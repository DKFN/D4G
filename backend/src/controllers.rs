use actix_files::NamedFile;
use crate::{LoginQuery};
use postgres::{Connection, TlsMode};
use serde_json::Value;
use serde_json::json;
use crate::model::{Logement, Proprietaire, Locataire, Releve};

pub fn index() -> Result<NamedFile, actix_web::Error> {
    let path = "./public/front/index.html";
    Ok(NamedFile::open(path)?)
}

pub fn connect_ddb() -> Connection{
    Connection::connect("postgresql://d4g:Design4Green@172.17.0.3:5432", TlsMode::None).unwrap()
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

    let result = Logement {
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
    };

    println!("{}", serde_json::to_string(&result).unwrap());
    result
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
                let user_foyer: String = row.get(1);
                let mut result: Logement = retrive_logement_from_foyer(&user_foyer);
                result.releves = retrive_releves_from_foyer(&user_foyer);

                ret = json!({"topic": "ok-login", "data" : result});
            }
        } else {
            ret = json!({ "topic": "ko-login", "data": { "message": "Account not validated" }});
        }
    } else {
            ret = json!({ "topic": "ko-login", "data": { "message": "Username or password incorrect" }});
        }
    ret
}
