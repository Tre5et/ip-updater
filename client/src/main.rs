use std::fs;
use std::path::Path;
use std::time::Duration;
use async_std::task;
use once_cell::sync::OnceCell;

static CHECK_URL: OnceCell<String> = OnceCell::new();
static UPDATE_URL: OnceCell<String> = OnceCell::new();
static PASSWORD: OnceCell<String> = OnceCell::new();
static INTERVAL: OnceCell<u64> = OnceCell::new();

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let path = Path::new("updater.conf");
    if !path.is_file() {
        return Err(std::io::Error::new(std::io::ErrorKind::NotFound, "Config not found!"));
    }
    let contents = fs::read_to_string(path).expect("Unable to read config!");
    let lines = contents.lines();
    for l in lines {
        if l.starts_with("check_url=") {
            let value = l.replace("check_url=", "");
            CHECK_URL.set(value).expect("Unable to set check url!");
        }
        if l.starts_with("update_url=") {
            let value = l.replace("update_url=", "");
            UPDATE_URL.set(value).expect("Unable to set update url!");
        }
        if l.starts_with("password=") {
            let value = l.replace("password=", "");
            PASSWORD.set(value).expect("Unable to set password!");
        }
        if l.starts_with("interval=") {
            let value = l.replace("interval=", "").parse::<u64>().expect("Unable to parse inverval!");
            INTERVAL.set(value).expect("Unable to set interval!");
        }
    }
    if CHECK_URL.get().is_none() || UPDATE_URL.get().is_none() || PASSWORD.get().is_none() || INTERVAL.get().is_none() {
        return Err(std::io::Error::new(std::io::ErrorKind::InvalidData, "Invalid config!"));
    }

    req_loop().await;

    return Ok(());
}

async fn req_loop() {
    let client = reqwest::Client::new();

    let mut prev_ip = "0".to_string();

    loop {
        let req = client.get(CHECK_URL.get().expect("No check url set"))
            .send().await;
        if req.is_ok() {
            let res = req.unwrap().text().await;
            if res.is_ok() {
                let ip = res.unwrap();

                if ip != prev_ip {
                    println!("Requesting new ip {}", ip);

                    let new_req = client.post(UPDATE_URL.get().expect("No update url set"))
                        .header("Content-Length", "0")
                        .header("User-Agent", "ip-updater")
                        .header("auth", PASSWORD.get().expect("No password set"))
                        .header("new-target-ip", ip.clone())
                        .body("")
                        .send().await;

                    if new_req.is_ok() {
                        let status = new_req.unwrap().status();
                        if status.is_success() {
                            println!("Updated ip");
                            prev_ip = ip.clone();
                        } else {
                            println!("Update server failure");
                        }
                    } else {
                        println!("Update failure");
                    }

                }
            } else {
                println!("Request test failure")
            }
        } else {
            println!("Request failure");
        }

        task::sleep(Duration::from_secs(*INTERVAL.get().expect("No interval set"))).await
    }
}
