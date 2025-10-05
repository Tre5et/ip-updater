extern crate core;

use std::fs;
use std::path::Path;
use actix_web::{App, HttpServer};
use once_cell::sync::OnceCell;

mod update;
mod service;

static PASSWORD: OnceCell<String> = OnceCell::new();
static PORT: OnceCell<u32> = OnceCell::new();
static PORTS: OnceCell<String> = OnceCell::new();

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let path = Path::new("updater.conf");
    if !path.is_file() {
        return Err(std::io::Error::new(std::io::ErrorKind::NotFound, "Config not found!"));
    }
    let contents = fs::read_to_string(path).expect("Unable to read config!");
    let lines = contents.lines();
    for l in lines {
        if l.starts_with("port=") {
            let value = l.replace("port=", "").parse::<u32>().expect("Unable to parse port!");
            PORT.set(value).expect("Unable to set port!");
        }
        if l.starts_with("password=") {
            let value = l.replace("password=", "");
            PASSWORD.set(value).expect("Unable to set password!");
        }
        if l.starts_with("ports=") {
            let value = l.replace("ports=", "");
            PORTS.set(value).expect("Unable to set ports");
        }
    }
    if PORT.get().is_none() || PASSWORD.get().is_none() || PORTS.get().is_none() {
        return Err(std::io::Error::new(std::io::ErrorKind::InvalidData, "Invalid config!"));
    }

    HttpServer::new(|| {
        App::new()
            .service(update::main)
    })
        .bind(format!("0.0.0.0:{p}", p = PORT.get().expect("Port not set!")))?
        .run()
        .await
}

pub fn get_password() -> String {
    return PASSWORD.get().expect("Passowrd not set!").clone();
}
pub fn get_ports() -> String { return PORTS.get().expect("File not set!").clone() }
