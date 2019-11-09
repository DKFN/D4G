extern crate env_logger;
#[macro_use] extern crate serde_derive;
use actix_files as fs;
use actix_web::{App, HttpServer, middleware, web};
use actix_files::NamedFile;
use json::JsonValue;
use serde_json::json;
use actix_web::web::Json;
use actix_web::middleware::Logger;

// "Model" Json et Database
#[derive(Deserialize, Serialize)]
pub struct ModelDeFou {
    name: String,
}

// "Controlleur"
pub fn greet() -> Result<Json<ModelDeFou>, actix_web::Error> {
    Ok(Json(ModelDeFou { name: "COUCOU".to_string() }))
}

// autre controlleur :)
pub fn index() -> Result<NamedFile, actix_web::Error> {
    let path = "./public/front/index.html";
    Ok(NamedFile::open(path)?)
}

pub fn main() {

    let app_port = std::env::var("APP_PORT").unwrap_or("8080".to_string());

    let splash = "
   _____ __  ________________  ____    _______   ______________   __________
  / ___// / / / ____/ ____/ / / / /   / ____/ | / / ____/ ____/  /  _/_  __/
  \\__ \\/ / / / /   / /   / / / / /   / __/ /  |/ / /   / __/     / /  / /
 ___/ / /_/ / /___/ /___/ /_/ / /___/ /___/ /|  / /___/ /___   _/ /  / /
/____/\\____/\\____/\\____/\\____/_____/_____/_/ |_/\\____/_____/  /___/ /_/
              ";
    println!("{}", splash);
    println!("[D4G] Web server launched o/");
    println!("[D4G] Docker inner port : 8080");
    println!("[D4G] UI Access http://localhost/");
    std::env::set_var("RUST_LOG", "actix_web=debug");
    env_logger::init();
    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Compress::default())
            .route("/", web::get().to(index))
            .route("/api/test", web::get().to(greet))
            .wrap(Logger::default())
    })
        .bind(format!("0.0.0.0:{}", app_port))
        .unwrap()
        .run()
        .unwrap();
}

