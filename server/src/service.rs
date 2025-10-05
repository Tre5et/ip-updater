use std::str;
use std::process::{Command};
use actix_web::HttpResponse;
use crate::{get_ports};

pub fn restart_redirect(ip: &str) -> Result<(), HttpResponse> {
    let mut comm = &mut Command::new("./configure-proxy".to_string());
    comm = comm.arg(ip.to_string());
    for p in get_ports().split(",") {
        comm = comm.arg(p.to_string());
    }
    let out = comm.output();
    if out.is_err() {
        println!("Failed to stop redirect");
        return Err(HttpResponse::InternalServerError().body("Failed to stop redirect!"))
    }

    let out = comm.spawn();
    if out.is_err() {
        println!("Failed to start proxy restart");
        return Err(HttpResponse::InternalServerError().body("Failed to start proxy restart!"));
    }
    let wait = out.unwrap().try_wait();
    if wait.is_err() {
        println!("Failed to wait for proxy restart");
        return Err(HttpResponse::InternalServerError().body("Failed to wait for proxy restart!"))
    }
    let wait = wait.unwrap();
    if wait.is_none() {
        println!("Failed to get proxy restart exit status");
        return Err(HttpResponse::InternalServerError().body("Failed to get proxy restart exit status"));
    }
    let wait = wait.unwrap();
    if !wait.success() {
        println!("Proxy restart exited with failure code");
        return Err(HttpResponse::InternalServerError().body("Proxy restart exited with failure code"));
    }

    return Ok(());
}