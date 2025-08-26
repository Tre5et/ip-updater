use std::fs;
use std::str;
use std::path::Path;
use std::process::Command;
use actix_web::HttpResponse;
use crate::{get_file, get_regex, get_start_cmd, get_stop_cmd};

pub fn restart_redirect(ip: &str) -> Result<(), HttpResponse> {
    let cmd = get_stop_cmd().clone();
    let args: Vec<&str> = cmd.split(" ").collect();
    println!("{:?}", args);
    let mut comm = &mut Command::new(args.get(0).unwrap());
    for i in 1..args.len() {
        comm = comm.arg(args.get(i).unwrap());
    }
    let out = comm.output();
    if out.is_err() {
        return Err(HttpResponse::InternalServerError().body("Failed to stop redirect!"))
    }
    let unwrap = out.unwrap();
    println!("{}", str::from_utf8(unwrap.stdout.as_ref()).unwrap_or(""));
    println!("{}", str::from_utf8(unwrap.stderr.as_ref()).unwrap_or(""));

    let file = get_file().clone();
    let path = Path::new(file.as_str());
    if !path.is_file() {
        return Err(HttpResponse::InternalServerError().body("File not found!"));
    }

    let file_contents = fs::read_to_string(path);
    if file_contents.is_err() {
        return Err(HttpResponse::InternalServerError().body("Unable to read file!"));
    }
    let file_content = file_contents.unwrap();

    let regex = get_regex();
    let res = regex.replace(file_content.as_str(), ip).to_string();

    let write = fs::write(path, res.as_str());
    if write.is_err() {
        return Err(HttpResponse::InternalServerError().body("Unable to write file!"));
    }

    let cmd = get_start_cmd().clone();
    let args: Vec<&str> = cmd.split(" ").collect();
    println!("{:?}", args);
    let mut comm = &mut Command::new(args.get(0).unwrap());
    for i in 1..args.len() {
        comm = comm.arg(args.get(i).unwrap());
    }
    let out = comm.spawn();
    if out.is_err() {
        return Err(HttpResponse::InternalServerError().body("Failed to start redirect!"));
    }
    //let unwrap = out.unwrap();
    //println!("{}", str::from_utf8(unwrap.stdout.as_ref()).unwrap_or(""));
    //println!("{}", str::from_utf8(unwrap.stderr.as_ref()).unwrap_or(""));

    return Ok(());
}