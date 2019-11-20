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
    let res = !conn.prepare("SELECT active FROM utilisateur where login=$1 AND password=$2").unwrap()
        .query(&[&query.login, &query.password]).unwrap().is_empty();
    res
}
