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
use crate::controllers::{index, login, register, sources};
use crate::model::{Logement};

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
pub struct RegisterQuery {
    login: String,
    password: String,
    logement: Logement
}

// do websocket handshake and start actor
fn ws_index(req: HttpRequest, stream: web::Payload) -> Result<HttpResponse, actix_web::Error> {
    ws::start(Ws, &req,  stream)
}

struct Ws;

impl Actor for Ws {
    type Context = ws::WebsocketContext<Self>;
}

// Handler for ws::Message messages
impl StreamHandler<ws::Message, ws::ProtocolError> for Ws {

    fn handle(&mut self, msg: ws::Message, ctx: &mut Self::Context) {
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
                        let response: Value = login(serde_json::from_value(request.data).unwrap());
                        ctx.text(response.to_string());
                    },
                    "register" => {
                        let data: RegisterQuery = serde_json::from_value(request.data).unwrap();
                        let response = register(data.login, data.password, data.logement);

                        ctx.text(json!({ "topic": "register", "data": { "message": response }}).to_string());
                    }
                    _ => {} // Needed so compiler don't end up in error
                }
            },
            ws::Message::Binary(bin) => ctx.binary(bin),
            _ => (),
        }
    }
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
            .wrap(middleware::Compress::default())
            .wrap(Logger::default())
            .route("/", web::get().to(index))
            .route("/source.zip", web::get().to(sources))
            .route("/socket", web::get().to(ws_index))
    })
        .bind(format!("0.0.0.0:{}", app_port))
        .unwrap()
        .run()
        .unwrap();
}
