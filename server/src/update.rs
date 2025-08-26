use actix_web::{post, HttpRequest, HttpResponse};
use crate::get_password;
use crate::service::restart_redirect;

#[post("/")]
pub async fn main(req: HttpRequest) -> HttpResponse {
    let auth_header = req.headers().get("auth");
    if !auth_header.is_some() {
        return HttpResponse::Forbidden().finish();
    }
    let auth_unwrap = auth_header.unwrap().to_str();
    if !auth_unwrap.is_ok() {
        return HttpResponse::Forbidden().finish();
    }
    let auth = auth_unwrap.unwrap();
    if !auth.eq(get_password().as_str()) {
        return HttpResponse::Forbidden().finish();
    }

    let ip_header = req.headers().get("new-target-ip");
    if !ip_header.is_some() {
        return HttpResponse::BadRequest().body("No 'new-target-ip' header.");
    }
    let ip_unwrap = ip_header.unwrap().to_str();
    if !ip_unwrap.is_ok() {
        return HttpResponse::BadRequest().body("Invalid 'new-target-ip' header.");
    }
    let ip = ip_unwrap.unwrap();

    let res = restart_redirect(ip);
    if res.is_err() {
        return res.unwrap_err();
    }

    return HttpResponse::Ok().body(format!("Restarted redirect with ip {}.", ip));
}