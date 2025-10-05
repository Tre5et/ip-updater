use std::str;
use std::process::{Command};
use actix_web::HttpResponse;
use crate::{get_ports};

pub fn restart_redirect(ip: &str) -> Result<(), HttpResponse> {
    let mut comm = &mut Command::new("sh".to_string());
    comm = comm.arg("configure-proxy".to_string());
    comm = comm.arg(ip.to_string());
    for p in get_ports().split(",") {
        comm = comm.arg(p.to_string());
    }
    let out = comm.output();
    if out.is_err() {
        println!("Failed to stop redirect: {}", out.unwrap_err());
        return Err(HttpResponse::InternalServerError().body("Failed to stop redirect!"))
    }

    let out = comm.spawn();
    if out.is_err() {
        println!("Failed to start proxy restart: {}", out.unwrap_err());
        return Err(HttpResponse::InternalServerError().body("Failed to start proxy restart!"));
    }
    let wait = out.unwrap().wait_with_output();
    if wait.is_err() {
        println!("Failed to wait for proxy restart: {}", wait.unwrap_err());
        return Err(HttpResponse::InternalServerError().body("Failed to wait for proxy restart!"))
    }
    let wait = wait.unwrap();
    println!("{}", String::from_utf8(wait.stdout).unwrap().to_string());
    if !wait.status.success() {
        println!("Proxy restart exited with failure code: {}, stderr:\n{}", wait.status, String::from_utf8(wait.stderr).unwrap().to_string());
        return Err(HttpResponse::InternalServerError().body("Proxy restart exited with failure code"));
    }

    return Ok(());
}