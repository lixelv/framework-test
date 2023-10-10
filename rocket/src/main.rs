#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

use rocket::http::Status;
use rocket::response::status;
use std::net::SocketAddr;

#[get("/get-ip")]
fn return_ip(remote_addr: Option<SocketAddr>) -> status::Custom<String> {
    match remote_addr {
        Some(addr) => status::Custom(Status::Ok, format!("{{\"data\": {{\"ip\": \"{}\"}}}}", addr.ip())),
        None => status::Custom(Status::InternalServerError, "Could not retrieve IP address.".to_string()),
    }
}

fn main() {
    rocket::ignite().mount("/", routes![return_ip]).launch();
}
