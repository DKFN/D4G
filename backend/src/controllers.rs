use actix_files::NamedFile;
use crate::{model, LoginQuery};
use postgres::{Connection, TlsMode};
use serde_json::Value;
use serde_json::json;

pub fn index() -> Result<NamedFile, actix_web::Error> {
    let path = "./public/front/index.html";
    Ok(NamedFile::open(path)?)
}

pub fn connect_ddb() -> Connection{
    Connection::connect("postgresql://d4g:Design4Green@172.17.0.3:5432", TlsMode::None).unwrap()
}

pub fn login(query:LoginQuery) -> Value {
    let mut ret: Value = json!({
            "topic": "ko-login",
            "data": {
                "message": "Unhandled server exception"
            }
        });
    let conn = connect_ddb();
    let rows = conn.prepare("SELECT active, foyer FROM utilisateur where login=$1 AND password=$2").unwrap()
        .query(&[&query.login, &query.password]).unwrap();
    if !rows.is_empty() {
        let row = rows.get(0);
        let active : bool = row.get(0);
        if active {
            let foyer : String = row.get(1);
            let logement = conn.prepare("select type, surface, nb_piece, chauffage, date_construction, n_voie, voie1, code_postal, ville from logement l where l.foyer = $1;").unwrap()
                .query(&[&foyer]).unwrap();
            let locataire = conn.prepare("select nom, prenom from locataire l where l.foyer = $1;").unwrap()
                .query(&[&foyer]).unwrap();
            let proprietaire = conn.prepare("select nom, prenom, societe, adresse from proprietaire p where p.foyer = $1;").unwrap()
                .query(&[&foyer]).unwrap();
            let releves = conn.prepare("select date, valeur from releve where foyer = $1").unwrap()
                .query(&[&foyer]).unwrap();

            ret = json!({"topic": "ok-login"});
        } else {
            ret = json!({ "topic": "ko-login", "data": { "message": "Account not validated" }});
        }
    } else {
            ret = json!({ "topic": "ko-login", "data": { "message": "Username or password incorrect" }});
        }
    ret
}
