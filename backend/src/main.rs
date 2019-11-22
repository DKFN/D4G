extern crate env_logger;
extern crate postgres;
#[macro_use] extern crate serde_derive;
use actix_web::{App, HttpServer, middleware, web};
use actix_web::middleware::Logger;
use actix_web::{HttpRequest, HttpResponse};
use actix_web_actors::ws;
use actix;
use actix::{StreamHandler, Actor};
use serde_json::Value;
use serde_json::json;
use crate::model::{Logement, AddReleve, Resume};
use crate::controllers::{index, login, register, sources, upload, verify, info_logement, user_retrieve_datas_from_polling, forget_password, add_releve, retrive_logement_admin, renew_password};
use std::cell::Cell;
use actix_files as afs;

mod controllers;
mod model;

#[derive(Deserialize, Serialize)]
pub struct SocketMessage {
    topic: String,
    data: Value
}

#[derive(Deserialize, Serialize)]
pub struct LoginQuery {
    login: String,
    password: String
}

#[derive(Deserialize, Serialize)]
pub struct ForgetPassword {
    login: String
}

#[derive(Deserialize, Serialize)]
pub struct InfoLogement {
    foyer: Option<String>
}

#[derive(Deserialize, Serialize)]
pub struct RegisterQuery {
    login: String,
    password: String,
    logement: Logement
}

#[derive(Deserialize)]
pub struct ForgetPasswordQuery {
    login: String
}

#[derive(Deserialize)]
pub struct RenewPasswordQuery {
    token: String,
    password: String
}

// do websocket handshake and start actor
fn ws_index(req: HttpRequest, stream: web::Payload) -> Result<HttpResponse, actix_web::Error> {
    ws::start(Ws { uname: "".to_string(), is_admin: false, foyers: vec![], auth: false, latest_sent: "".to_string() }, &req, stream)
}

pub struct Ws {
    pub uname: String,
    pub is_admin: bool,
    pub foyers: Vec<String>,
    pub auth: bool,
    pub latest_sent: String,
}

impl Actor for Ws {
    type Context = ws::WebsocketContext<Self>;
}


// Handler for ws::Message messages
impl StreamHandler<ws::Message, ws::ProtocolError> for Ws {

    fn handle(&mut self, msg: ws::Message, ctx: &mut Self::Context) {
        println!("RQ CTX {} | AUTH ? {} |  ADMIN ? {}", self.uname, self.auth, self.is_admin);
        match msg {
            ws::Message::Ping(msg) => {
                println!("Ping {}", msg);
                ctx.pong(&msg)
            },
            ws::Message::Text(text) => {
                println!("Text {}", text);

                let request: SocketMessage = serde_json::from_str(&text).unwrap();
                match request.topic.as_str() {
                    "try-login" => {
                        let data: LoginQuery = serde_json::from_value(request.data).unwrap();
                        let (response, connection_ok, is_admin): (Value, bool, bool) = login(&data);
                        if connection_ok {
                            self.uname = data.login.clone();
                            self.auth = true;
                            self.is_admin = is_admin.clone();
                            self.latest_sent = response["data"].to_string().clone();
                        }
                        ctx.text(response.to_string().clone());
                    },
                    "register" => {
                        if self.auth && self.is_admin {
                            let data: RegisterQuery = serde_json::from_value(request.data).unwrap();
                            let response = register(data.login, data.password, data.logement);

                            ctx.text(json!({ "topic": "register", "data": { "message": response }}).to_string());
                        } else {
                            ctx.text(json!({ "topic": "403", "data": { "message": "You are not authorized"}}).to_string());
                        }
                    },
                    "forget-password" => {
                        let data: ForgetPasswordQuery = serde_json::from_value(request.data).unwrap();
                        let (response, is_error) = forget_password(data.login);

                        if is_error {
                            ctx.text(json!({ "topic": "forget-password", "data": { "error": response }}).to_string());
                        } else {
                            ctx.text(json!({ "topic": "forget-password", "data": { "message": response }}).to_string());
                        }
                    },
                    "renew-password" => {
                        let data: RenewPasswordQuery = serde_json::from_value(request.data).unwrap();
                        let (response, is_error) = renew_password(data.token, data.password);

                        if is_error {
                            ctx.text(json!({ "topic": "renew-password", "data": { "error": response }}).to_string());
                        } else {
                            ctx.text(json!({ "topic": "renew-password", "data": { "message": response }}).to_string());
                        }
                    },
                    "info-logement" => {
                        let data: InfoLogement = serde_json::from_value(request.data).unwrap();
                        let response: Logement = info_logement(&data.foyer.unwrap());
                        self.latest_sent = serde_json::to_string(&response).unwrap().clone();
                        ctx.text(json!({ "topic": "ok-info", "data": response}).to_string());
                    },
                    "add-releve" => {
                        let data : AddReleve = serde_json::from_value(request.data).unwrap();
                        add_releve(&data);
                        ctx.text(json!({ "topic": "ok-add-releve", "data": { "message": "Releve ajouté avec succès"}}).to_string());
                    },
                    "poll-data" => {
                        if self.auth {
                            if self.is_admin {
                                println!("ADMIN POLL");
                                let data: InfoLogement = serde_json::from_value(request.data).unwrap();
                                // TODO: Check if foyer. If not then we need to poll info logements
                                if data.foyer.is_some() {
                                    let logement = data.foyer.unwrap();
                                    let response = info_logement(&logement);
                                    let cache_valid = self.latest_sent == serde_json::to_string(&response)
                                        .unwrap().clone();
                                    println!("CACHE VALID ? {}", cache_valid);
                                    if !cache_valid {
                                        self.latest_sent = serde_json::to_string(&response).unwrap().clone();
                                        ctx.text(json!({ "topic": "ok-info", "data": response}).to_string());
                                    }
                                } else {
                                    let result : Vec<Resume> = retrive_logement_admin();
                                    let cache_valid = self.latest_sent == serde_json::to_string(&result).unwrap().clone();
                                    println!("CACHE VALID ? {}", cache_valid);
                                    if !cache_valid {
                                        self.latest_sent = serde_json::to_string(&result).unwrap().clone();
                                        ctx.text(json!({ "topic": "poll-data", "data": result}).to_string());
                                    }
                                }
                            } else {
                                let uname = self.uname.clone();
                                let polled_datas = user_retrieve_datas_from_polling(uname);
                                let cache_valid = self.latest_sent == polled_datas["data"].to_string();
                                println!("CACHE VALID ? {}", cache_valid);
                                if !cache_valid {
                                    self.latest_sent = polled_datas["data"].to_string().clone();
                                    ctx.text(polled_datas.to_string());
                                }
                            }
                        } else {
                            ctx.text(json!({ "topic": "403", "data": { "message": "You are not authorized"}}).to_string());
                        }
                    }
                    _ => {} // Needed so compiler don't end up in error
                }
            },
            ws::Message::Binary(bin) => ctx.binary(bin),
            _ => (),
        }
    }
}

pub struct AppState {
    pub counter: Cell<usize>
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
    println!("[D4G] Docker inner port : {}", app_port);
    println!("[D4G] UI Access http://localhost/");
    std::env::set_var("RUST_LOG", "actix_web=debug");
    env_logger::init();

    HttpServer::new(|| {
        App::new()
            .data(Cell::new(0usize))
            .wrap(middleware::Compress::default())
            .wrap(Logger::default())
            .route("/", web::get().to(index))
            .service(web::resource("/verify/{token}").route(web::get().to(verify)))
            .route("/source.zip", web::get().to(sources))
            .route("/socket", web::get().to(ws_index))
            .route("/file/{foyer}", web::post().to_async(upload))
            .service(
                afs::Files::new("/files", "/public/uploads")
                    .show_files_listing()
                    .use_last_modified(true))
    })
        .bind(format!("0.0.0.0:{}", app_port))
        .unwrap()
        .run()
        .unwrap();
}
