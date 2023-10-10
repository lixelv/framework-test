use actix_web::{web, App, HttpRequest, HttpServer, Responder};

async fn return_ip(req: HttpRequest) -> impl Responder {
    let ip = req.peer_addr().unwrap().ip();
    format!("{{\"data\": {{\"ip\": \"{}\"}}}}", ip)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().route("/get-ip", web::get().to(return_ip))).bind("127.0.0.1:9004")?.run().await
}
