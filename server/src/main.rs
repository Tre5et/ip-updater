extern crate core;

use std::fs;
use std::path::Path;
use actix_web::{App, HttpServer};
use fancy_regex::Regex;
use once_cell::sync::OnceCell;

mod update;
mod service;

static PASSWORD: OnceCell<String> = OnceCell::new();
static PORT: OnceCell<u32> = OnceCell::new();
static FILE: OnceCell<String> = OnceCell::new();
static REGEX: OnceCell<Regex> = OnceCell::new();
static STOP_CMD: OnceCell<String> = OnceCell::new();
static START_CMD: OnceCell<String> = OnceCell::new();

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
        if l.starts_with("file=") {
            let value = l.replace("file=", "");
            FILE.set(value).expect("Unable to set file!");
        }
        if l.starts_with("regex=") {
            let value = l.replace("regex=", "");
            REGEX.set(Regex::new(value.as_str()).expect("Unable to compile regex!")).expect("Unable to set regex!");
        }
        if l.starts_with("stop_cmd=") {
            let value = l.replace("stop_cmd=", "");
            STOP_CMD.set(value).expect("Unable to set stop command!");
        }
        if l.starts_with("start_cmd=") {
            let value = l.replace("start_cmd=", "");
            START_CMD.set(value).expect("Unable to set start command!");
        }
    }
    if PORT.get().is_none() || PASSWORD.get().is_none() || FILE.get().is_none() || REGEX.get().is_none() || STOP_CMD.get().is_none() || START_CMD.get().is_none() {
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
pub fn get_file() -> String { return FILE.get().expect("File not set!").clone() }
pub fn get_regex() -> Regex { return REGEX.get().expect("Regex not set!").clone() }
pub fn get_stop_cmd() -> String { return STOP_CMD.get().expect("Stop command not set!").clone() }
pub fn get_start_cmd() -> String { return START_CMD.get().expect("Start command not set!").clone() }
