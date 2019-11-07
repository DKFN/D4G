extern crate env_logger;
use actix_files as fs;
use actix_web::{App, HttpServer};

pub fn main() {
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
        App::new().service(
            // This folder will be created on docker build
            fs::Files::new("/", "./public/front/").index_file("index.html")
        )
    })
        .bind("0.0.0.0:8080")
        .unwrap()
        .run()
        .unwrap();
}

