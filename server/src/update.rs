use actix_web::{post, HttpRequest, HttpResponse};
use crate::get_password;
use crate::service::restart_redirect;

#[post("/")]
pub async fn main(req: HttpRequest) -> HttpResponse {
    println!("New request");

    let auth_header = req.headers().get("auth");
    if !auth_header.is_some() {
        println!("Blocked request 1");
        return HttpResponse::Forbidden().finish();
    }
    let auth_unwrap = auth_header.unwrap().to_str();
    if !auth_unwrap.is_ok() {
        println!("Blocked request 2");
        return HttpResponse::Forbidden().finish();
    }
    let auth = auth_unwrap.unwrap();
    if !auth.eq(get_password().as_str()) {
        println!("Blocked request 3");
        return HttpResponse::Forbidden().finish();
    }

    let ip_header = req.headers().get("new-target-ip");
    if !ip_header.is_some() {
        println!("Bad request 1");
        return HttpResponse::BadRequest().body("No 'new-target-ip' header.");
    }
    let ip_unwrap = ip_header.unwrap().to_str();
    if !ip_unwrap.is_ok() {
        println!("Bad request 2");
        return HttpResponse::BadRequest().body("Invalid 'new-target-ip' header.");
    }
    let ip = ip_unwrap.unwrap();
    
    println!("Received request to update to {}.", ip);

    let res = restart_redirect(ip);
    if res.is_err() {
        return res.unwrap_err();
    }

    println!("Update successful.");

    return HttpResponse::Ok().body(format!("Restarted redirect with ip {}.", ip));
}