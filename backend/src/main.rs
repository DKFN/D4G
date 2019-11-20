extern crate env_logger;
extern crate postgres;
#[macro_use] extern crate serde_derive;
use actix_web::{App, HttpServer, middleware, web};
use actix_files::NamedFile;
use actix_web::web::Json;
use actix_web::middleware::Logger;
use actix_web::{HttpRequest, HttpResponse};
use actix_web_actors::ws;
use actix;
use actix::{StreamHandler, Actor};
use postgres::{Connection, TlsMode};
use json::JsonValue;
use serde_json::Value;
use crate::controllers::{index, login};
use serde_json::json;

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

// "Model" Json et Database
#[derive(Deserialize, Serialize)]
pub struct ModelDeFou {
    name: String,
}



// "Controlleur"
pub fn greet() -> Result<Json<ModelDeFou>, actix_web::Error> {
    Ok(Json(ModelDeFou { name: "COUCOU".to_string()}))
}

// autre controlleur :)



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
                println!("{}", msg);
                ctx.pong(&msg)
            },
            ws::Message::Text(text) => {
                println!("{}", text);
                let response: SocketMessage = serde_json::from_str(&text).unwrap();
                println!("{}", response.topic);
                if (response.topic == "try-login") {
                    let data = response.data.to_string();
                    println!("DATA: {}", data);
                    let response: LoginQuery = serde_json::from_str(&data).unwrap();
                    let result = login(response);
                    ctx.text(json!({
                        "topic": if result { "ok-login" } else { "ko-login" }
                    }).to_string());
                }
                ctx.text(text)
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
    println!("[D4G] Docker inner port : 8080");
    println!("[D4G] UI Access http://localhost/");
    std::env::set_var("RUST_LOG", "actix_web=debug");
    env_logger::init();
    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Compress::default())
            .route("/", web::get().to(index))
            .route("/socket", web::get().to(ws_index))
            .route("/api/test", web::get().to(greet))
            .wrap(Logger::default())
    })
        .bind(format!("0.0.0.0:{}", app_port))
        .unwrap()
        .run()
        .unwrap();
}
