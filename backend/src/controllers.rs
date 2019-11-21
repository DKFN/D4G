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
use crate::model::{Logement, Proprietaire, Locataire, Releve, Resume};
use lettre::{ ClientSecurity, ClientTlsParameters, SmtpClient, Transport };
use lettre::smtp::authentication::{Credentials, Mechanism};
use lettre::smtp::ConnectionReuseParameters;
use native_tls::{Protocol, TlsConnector};
use lettre_email::{Email};
use regex::Regex;

pub fn index() -> Result<NamedFile, actix_web::Error> {
    let path = "./public/front/index.html";
    Ok(NamedFile::open(path)?)
}

pub fn sources() -> Result<NamedFile, actix_web::Error> {
    let path = "./public/front/source.zip";
    Ok(NamedFile::open(path)?)
}

pub fn connect_ddb() -> Connection{
    Connection::connect("postgresql://d4g:Design4Green@172.17.0.3:5432", TlsMode::None).unwrap()
}

pub fn register(username: String, password: String, logement: Logement) -> String {
    let conn = connect_ddb();
    let result;

    let row = conn.prepare("SELECT active FROM utilisateur where login=$1").unwrap()
        .query(&[&username]).unwrap();

    if !row.is_empty() {
        result = "User already exist".to_string();
    } else {
        let token = nanoid::simple();
        let re = Regex::new(r"^[a-zA-Z0-9.!#$%&'*+/=?^_`{|}~-]+@[a-zA-Z0-9](?:[a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?(?:\.[a-zA-Z0-9](?:[a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?)*$").unwrap();
        let username_is_email = re.is_match(&username);

        if username_is_email {
            let domain = std::env::var("DOMAIN").unwrap_or("localhost".to_string());
            let address = std::env::var("SMTP_ADDRESS").unwrap_or("smtp.gmail.com".to_string());

            let email = Email::builder()
                .to(username.clone())
                .from(("bot@vps753500.ovh.net", "Green Jiraration"))
                .subject("Hi, activate your account")
                .text(format!("http://{}/verify/{}", domain, token))
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

            if result.is_ok() {
                println!("Email sent");
            } else {
                println!("Could not send email: {:?}", result);
            }

            mailer.close();
        }

        let foyer = nanoid::generate(16); // We generate an id of 16 char because of database typing
        conn.prepare("INSERT INTO logement VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)").unwrap()
            .query(&[&foyer, &logement.l_type, &logement.surface, &logement.nb_pieces, &logement.chauffage, &logement.date_construction, &logement.n_voie, &logement.voie1, &logement.code_postal, &logement.ville]).unwrap();

        conn.prepare("INSERT INTO proprietaire VALUES ($1, $2, $3, $4, $5)").unwrap()
            .query(&[&foyer, &logement.proprietaire.nom, &logement.proprietaire.prenom, &logement.proprietaire.societe, &logement.proprietaire.adresse]).unwrap();

        conn.prepare("INSERT INTO locataire VALUES ($1, $2, $3)").unwrap()
            .query(&[&foyer, &logement.locataire.nom, &logement.locataire.prenom]).unwrap();

        conn.prepare("INSERT INTO utilisateur VALUES ($1, $2, $3, $4, $5)").unwrap()
            .query(&[&foyer, &username, &password, &!username_is_email, &token]).unwrap();

        result = "Check your mail to verify your account".to_string()
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
        Resume {
            foyer: row.get(0),
            l_type: row.get(1),
            ville: row.get(2),
            proprietaire_nom: row.get(3),
            proprietaire_prenom: row.get(4),
            proprietaire_societe: row.get(5),
            locataire_nom: row.get(6),
            locataire_prenom: row.get(7),
        }
    }).collect()
}
pub fn login(query:LoginQuery) -> Value {
    let mut ret: Value = json!({
            "topic": "ko-login",
            "data": {
                "message": "Unhandled server exception"
            }
        });
    let conn = connect_ddb();
    let rows = conn.prepare("SELECT active, foyer, admin FROM utilisateur where login=$1 AND password=$2").unwrap()
        .query(&[&query.login, &query.password]).unwrap();
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
            }
        } else {
            ret = json!({ "topic": "ko-login", "data": { "message": "Account not validated" }});
        }
    } else {
            ret = json!({ "topic": "ko-login", "data": { "message": "Username or password incorrect" }});
        }
    ret
}

// D4G2019  vTbKanJFMiToP
