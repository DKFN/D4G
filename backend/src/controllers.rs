extern crate native_tls;
extern crate nanoid;
extern crate lettre;
extern crate lettre_email;
extern crate regex;

use actix_files::NamedFile;
use crate::{LoginQuery};
use postgres::{Connection, TlsMode};
use serde_json::Value;
use serde_json::json;
use crate::model::{Logement, Proprietaire, Locataire, Releve, Resume, AddReleve};
use lettre::{ ClientSecurity, ClientTlsParameters, SmtpClient, Transport };
use lettre::smtp::authentication::{Credentials, Mechanism};
use lettre::smtp::ConnectionReuseParameters;
use native_tls::{Protocol, TlsConnector};
use lettre_email::{Email};
use regex::Regex;
use actix_web::{web, HttpResponse, Error, error, HttpRequest};
use actix_web::http::{header, StatusCode};
use std::cell::Cell;
use futures::{Future, Stream};
use futures::future::{Either, err};
use std::fs;
use actix_multipart::{Field, Multipart, MultipartError};
use std::io::Write;
use crypto::digest::Digest;
use crypto::sha2::Sha256;

fn hash(password: String) -> String {
    let mut hasher = Sha256::new();
    hasher.input_str(password.as_str());
    hasher.result_str()
}

pub fn index() -> Result<NamedFile, actix_web::Error> {
    let path = "./public/front/index.html";
    Ok(NamedFile::open(path)?)
}

pub fn sources() -> Result<NamedFile, actix_web::Error> {
    let path = "./public/front/source.zip";
    Ok(NamedFile::open(path)?)
}

pub fn verify(_req: HttpRequest, path: web::Path<(String,)>) -> HttpResponse {
    let token = path.0.clone();
    let conn = connect_ddb();

    let rows = conn.prepare("SELECT active FROM utilisateur where token=$1").unwrap()
        .query(&[&token]).unwrap();

    if !rows.is_empty() {
        conn.prepare("UPDATE utilisateur SET active = $1, token = NULL where token=$2").unwrap()
            .query(&[&true, &token]).unwrap();
    }

    HttpResponse::build(StatusCode::TEMPORARY_REDIRECT)
        .header(header::LOCATION, "/")
        .finish()
}

pub fn connect_ddb() -> Connection{
    Connection::connect("postgresql://d4g:Design4Green@172.17.0.3:5432", TlsMode::None).unwrap()
}

pub fn send_email(subject: String, mail: String, message: String) -> lettre::smtp::error::SmtpResult {
    let address = std::env::var("SMTP_ADDRESS").unwrap_or("smtp.gmail.com".to_string());

    let email = Email::builder()
        .to(mail)
        .from(("bot@vps753500.ovh.net", "Green Jiraration"))
        .subject(subject)
        .text(message)
        .build()
        .unwrap();

    let mut tls_builder = TlsConnector::builder();
    tls_builder.min_protocol_version(Some(Protocol::Tlsv10));

    let tls_parameters = ClientTlsParameters::new(
        address.to_string(),
        tls_builder.build().unwrap()
    );

    let mut mailer = SmtpClient::new(
        (address.as_str(), 465),
        ClientSecurity::Wrapper(tls_parameters)
    ).unwrap()
        .authentication_mechanism(Mechanism::Login)
        .credentials(Credentials::new(
            std::env::var("SMTP_USERNAME").unwrap_or("user".to_string()),
            std::env::var("SMTP_PASSWORD").unwrap_or("password".to_string())
        ))
        .connection_reuse(ConnectionReuseParameters::ReuseUnlimited)
        .transport();

    let result = mailer.send(email.into());

    mailer.close();

    if !result.is_ok() {
        println!("Could not send email: {:?}", result);
    }

    result
}

fn is_email(login: String) -> bool {
    let re = Regex::new(r"^[a-zA-Z0-9.!#$%&'*+/=?^_`{|}~-]+@[a-zA-Z0-9](?:[a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?(?:\.[a-zA-Z0-9](?:[a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?)*$").unwrap();
    re.is_match(&login)
}

pub fn renew_password(token: String, password: String) -> (String, bool) {
    let mut is_error = true;
    let result;

    let conn = connect_ddb();
    let rows = conn.prepare("SELECT active FROM utilisateur where token=$1").unwrap()
        .query(&[&token]).unwrap();

    if rows.is_empty() {
        result = "Impossible de trouver la demande de réinitialisation de mot de passe".to_string();
    } else {
        conn.prepare("UPDATE utilisateur SET password = $1, token = NULL where token=$2").unwrap()
            .query(&[&hash(password), &token]).unwrap();

        is_error = false;
        result = "Votre mot de passe a été changé, vous pouvez vous connecter à nouveau".to_string();
    }

    (result, is_error)
}

pub fn forget_password(login: String) -> (String, bool) {
    let email = std::env::var("SMTP_USERNAME").unwrap_or("l'administrateur".to_string());
    let mut is_error = true;
    let mut result = format!("Nous n'avons pas réussi à vous contacter, envoyer un mail à {}", email);


    if is_email(login.clone()) {
        let conn = connect_ddb();
        let rows = conn.prepare("SELECT active FROM utilisateur where login=$1").unwrap()
            .query(&[&login]).unwrap();

        if rows.is_empty() {
            result = "Ce nom d'utilisateur est inconnu".to_string();
        } else {
            let activated: bool = rows.get(0).get(0);
            if !activated {
                result = format!("Votre compte n'est pas activé, vérifiez votre boîte mail ou contactez {}", email);
            } else {
                let domain = std::env::var("DOMAIN").unwrap_or("localhost".to_string());
                let token = nanoid::simple();

                conn.prepare("UPDATE utilisateur SET token = $1 where login=$2").unwrap()
                    .query(&[&token, &login]).unwrap();

                let mail = send_email("Changer votre mot de passe".to_string(), login.clone(), format!("http://{}?token={}", domain, token));

                if mail.is_ok() {
                    result = "Un mail pour réinitialisez votre mot de passe viens de vous être envoyé".to_string();
                    is_error = false;
                }
            }
        }
    }

    (result, is_error)
}

pub fn register(username: String, password: String, logement: Logement) -> String {
    let conn = connect_ddb();
    let mut result;

    let rows = conn.prepare("SELECT active FROM utilisateur where login=$1").unwrap()
        .query(&[&username]).unwrap();

    if !rows.is_empty() {
        result = "L'utilisateur existe déjà".to_string();
    } else {
        let token = nanoid::simple();
        let foyer = nanoid::generate(16); // We generate an id of 16 char because of database typing
        let username_is_email = is_email(username.clone());

        conn.prepare("INSERT INTO logement VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)").unwrap()
            .query(&[&foyer, &logement.l_type, &logement.surface, &logement.nb_pieces, &logement.chauffage, &logement.date_construction, &logement.n_voie, &logement.voie1, &logement.code_postal, &logement.ville]).unwrap();

        conn.prepare("INSERT INTO proprietaire VALUES ($1, $2, $3, $4, $5)").unwrap()
            .query(&[&foyer, &logement.proprietaire.nom, &logement.proprietaire.prenom, &logement.proprietaire.societe, &logement.proprietaire.adresse]).unwrap();

        conn.prepare("INSERT INTO locataire VALUES ($1, $2, $3)").unwrap()
            .query(&[&foyer, &logement.locataire.nom, &logement.locataire.prenom]).unwrap();

        conn.prepare("INSERT INTO utilisateur VALUES ($1, $2, $3, $4, $5, $6)").unwrap()
            .query(&[&foyer, &username, &hash(password), &!username_is_email, &token, &false]).unwrap();

        result = "Un email vous a été envoyé pour vérifier votre compte".to_string();

        if username_is_email {
            let domain = std::env::var("DOMAIN").unwrap_or("localhost".to_string());
            let mail = send_email("Activez votre compte !".to_string(), username.clone(), format!("http://{}/verify/{}", domain, token));

            if !mail.is_ok() {
                result = "Une erreur est survenue lors de l'envoi de l'email".to_string();
            }
        }
    }

    result
}

pub fn retrive_logement_from_foyer(user_foyer: &String) -> Logement {
    let conn = connect_ddb();
    let logements = conn.prepare("select type, surface, nb_pieces, chauffage, date_construction, n_voie, voie1, code_postal, ville from logement l where l.foyer = $1;").unwrap()
        .query(&[user_foyer]).unwrap();
    let locataires = conn.prepare("select nom, prenom from locataire l where l.foyer = $1;").unwrap()
        .query(&[user_foyer]).unwrap();
    let proprietaires = conn.prepare("select nom, prenom, societe, adresse from proprietaire p where p.foyer = $1;").unwrap()
        .query(&[user_foyer]).unwrap();
    let files = get_files_for_foyer(user_foyer);


    let logement = logements.get(0);
    let locataire = locataires.get(0);
    let proprietaire = proprietaires.get(0);

    Logement {
        foyer: user_foyer.clone().to_string().parse().unwrap(),
        l_type: logement.get(0),
        surface: logement.get(1),
        nb_pieces: logement.get(2),
        chauffage: logement.get(3),
        date_construction: logement.get(4),
        n_voie: logement.get(5),
        voie1: logement.get(6),
        code_postal: logement.get(7),
        ville: logement.get(8),
        proprietaire: Proprietaire {
            nom: proprietaire.get(0),
            prenom: proprietaire.get(1),
            societe: proprietaire.get(2),
            adresse: proprietaire.get(3),
        },
        locataire: Locataire {
            nom: locataire.get(0),
            prenom: locataire.get(1),
        },
        releves: vec![],
        fichiers: files
    }
}

pub fn retrive_releves_from_foyer(user_foyer: &String) -> Vec<Releve>{
    let conn = connect_ddb();
    let releves = conn.prepare("select date, valeur from releve where foyer = $1").unwrap()
        .query(&[user_foyer]).unwrap();
    releves.iter().map( | row | {
        Releve {
            date: row.get(0),
            valeur: row.get(1),
        }
    }).collect()
}

pub fn retrive_logement_admin() -> Vec<Resume>{
    let conn = connect_ddb();
    let rows = conn.prepare("select l.foyer, l.type, l.ville, p.nom, p.prenom, p.societe, l2.nom, l2.prenom  from logement l
                                            join locataire l2 on l.foyer = l2.foyer
                                            join proprietaire p on l.foyer = p.foyer
                                            order by l.foyer asc;").unwrap().query(&[]).unwrap();

    rows.iter().map( | row | {
        let maybe_societe: Option<String> = row.get(5);
        let nom: String = row.get(6);
        let prenom: String = row.get( 7);
        let pnom: Option<String> = row.get(3);
        let pprenom: Option<String> = row.get(4);
        Resume {
            foyer: row.get(0),
            l_type: row.get(1),
            ville: row.get(2),
            locataire: format!("{} {}", &nom, &prenom),
            proprietaire: maybe_societe.unwrap_or(format!("{} {}", &pnom.unwrap_or_default(), &pprenom.unwrap_or_default())),
        }
    }).collect()
}

pub fn info_logement(query: &String) -> Logement {
    let mut result: Logement = retrive_logement_from_foyer(&query);
    result.releves = retrive_releves_from_foyer(&query);
    result
}

pub fn delete_all(foyer: &String, uname: &String){
    let conn = connect_ddb();
    conn.prepare("DELETE FROM logement where foyer = $1").unwrap().query(&[foyer]).unwrap();
    conn.prepare("DELETE FROM utilisateur where login = $1").unwrap().query(&[uname]).unwrap();
}

pub fn delete_logement(foyer: &String){
    let conn = connect_ddb();
    conn.prepare("DELETE FROM logement where foyer = $1").unwrap().query(&[foyer]).unwrap();
}

pub fn add_releve(query: &AddReleve) {
    let conn = connect_ddb();
    conn.prepare("INSERT INTO releve (foyer, date, valeur) VALUES ($1, $2, $3);").unwrap()
        .query(&[&query.foyer, &query.date, &query.valeur]).unwrap();
}

pub fn login(query: &LoginQuery) -> (Value, bool, bool) {
    // TODO: Add tuple as return to have all datas to give to ctx
    let ret: Value;
    let mut is_admin = false;
    let mut conn_ok = false;
    let conn = connect_ddb();
    let rows = conn.prepare("SELECT active, foyer, admin FROM utilisateur where login=$1 AND password=$2").unwrap()
        .query(&[&query.login, &hash(query.password.clone())]).unwrap();

    if !rows.is_empty() {
        let row = rows.get(0);
        let active : bool = row.get(0);
        if active {
            let admin : bool = row.get(2);
            if !admin {
                println!("Utilisateur connecte");
                let user_foyer: String = row.get(1);
                let mut result: Logement = retrive_logement_from_foyer(&user_foyer);
                result.releves = retrive_releves_from_foyer(&user_foyer);

                ret = json!({"topic": "ok-login", "data" : result});
            } else {
                println!("Admin connecte");
                let result : Vec<Resume> = retrive_logement_admin();
                ret = json!({"topic": "ok-login-admin", "data" : result});
                is_admin = true;
            }
            conn_ok = true;
        } else {
            ret = json!({ "topic": "ko-login", "data": { "message": "Account not validated" }});
        }
    } else {
            ret = json!({ "topic": "ko-login", "data": { "message": "Username or password incorrect" }});
        }
    (ret, conn_ok, is_admin)
}

pub fn get_files_for_foyer(foyer: &String) -> Vec<String> {
    let conn = connect_ddb();
    let mut res: Vec<String> = vec![];
    let rows = conn.prepare("SELECT file_path FROM fichier WHERE foyer = $1;").unwrap()
        .query(&[foyer]).unwrap();
    rows.iter().for_each(|f| -> (){
        res.push(f.get(0))
    });
    res
}

pub fn save_file_bdd(filepath: &String, foyer: &String) {
    let conn = connect_ddb();
    let ident = nanoid::simple();
    conn.prepare("INSERT INTO fichier (foyer, id, file_path) VALUES ($1, $2, $3);").unwrap()
        .query(&[foyer, &ident, filepath]).unwrap();
}

pub fn user_retrieve_datas_from_polling(ulogin: String) -> Value {
    let conn = connect_ddb();
    let rows = conn.prepare("SELECT active, foyer, admin FROM utilisateur where login=$1").unwrap()
        .query(&[&ulogin]).unwrap();

    let user_foyer = rows.get(0).get(1);
    foyer_retrieve_datas_from_polling(&user_foyer)
}

pub fn foyer_retrieve_datas_from_polling(user_foyer: &String) -> Value {
    let mut result: Logement = retrive_logement_from_foyer(&user_foyer);
    result.releves = retrive_releves_from_foyer(&user_foyer);

    json!({"topic": "poll-data", "data" : result})
}

// D4G2019  vTbKanJFMiToP
pub fn save_file(field: Field, foyer: String) -> impl Future<Item = i64, Error = Error> {

    // TODO: Generate something

    let filename: String = field.content_disposition().unwrap().get_filename().unwrap().to_string()
        .replace(' ', "_").to_string();
    let file_path_string = format!("/public/uploads/{}", filename);
    let exposed_path_string = format!("/files/{}", filename);
    save_file_bdd(&exposed_path_string, &foyer);
    let file = match fs::File::create(file_path_string) {
        Ok(file) => file,
        Err(e) => return Either::A(err(error::ErrorInternalServerError(e))),
    };
    Either::B(
        field
            .fold((file, 0i64), move |(mut file, mut acc), bytes| {
                // fs operations are blocking, we have to execute writes
                // on threadpool
                web::block(move || {
                    file.write_all(bytes.as_ref()).map_err(|e| {
                        println!("file.write_all failed: {:?}", e);
                        MultipartError::Payload(error::PayloadError::Io(e))
                    })?;
                    acc += bytes.len() as i64;
                    Ok((file, acc))
                })
                    .map_err(|e: error::BlockingError<MultipartError>| {
                        match e {
                            error::BlockingError::Error(e) => e,
                            error::BlockingError::Canceled => MultipartError::Incomplete,
                        }
                    })
            })
            .map(|(_, acc)| acc)
            .map_err(|e| {
                println!("save_file failed, {:?}", e);
                error::ErrorInternalServerError(e)
            }),
    )
}

pub fn upload(multipart: Multipart, counter: web::Data<Cell<usize>>, foyer : actix_web::web::Path<String>) -> impl Future<Item = HttpResponse, Error = Error> {
    counter.set(counter.get() + 1);
    println!("{:?}", counter.get());
    let q = foyer.into_inner();
    multipart
        .map_err(error::ErrorInternalServerError)
        .map(move |field| {
            let r = q.clone();
            save_file(field, r).into_stream()
        })
        .flatten()
        .collect()
        .map(|sizes| HttpResponse::Ok().json(sizes))
        .map_err(|e| {
            println!("failed: {}", e);
            e
        })
}
