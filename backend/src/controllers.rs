use actix_files::NamedFile;
use crate::{model, LoginQuery};
use postgres::{Connection, TlsMode};

pub fn index() -> Result<NamedFile, actix_web::Error> {
    let path = "./public/front/index.html";
    Ok(NamedFile::open(path)?)
}

pub fn connect_ddb() -> Connection{
    Connection::connect("postgresql://d4g:Design4Green@172.17.0.3:5432", TlsMode::None).unwrap()
}

pub fn login(query:LoginQuery) -> bool {
    let conn = connect_ddb();
    let rows = !conn.prepare("SELECT active, foyer FROM utilisateur where login=$1 AND password=$2").unwrap()
        .query(&[&query.login, &query.password]).unwrap();
    println!("login correct : {}", res);
    if !rows.is_empty() {
        if rows.get(0) {
            let infos = conn.prepare("select l.foyer from logement l where l.foyer = $1;").unwrap()
                .query(&[rows.get(2)]).unwrap();
            infos.iter().for_each(|r| -> () {
                let maybe_foyer: Option<String> = r.get(0);
                println!("infos du logement :{}", maybe_foyer.unwrap_or_default());
            });

            let releves = conn.prepare("select date, valeur from releve where foyer = $1").unwrap()
                .query(&rows.get(2)).unwrap();

            println!("infos du logement :{}", releves);
        } //else {
        //TODO gerer le fait que le compte soit non actif
        //}

    }
    true
}
