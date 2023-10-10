use tide::{Request, prelude::*};

#[async_std::main]
async fn main() -> tide::Result<()> {
    let mut app = tide::new();
    app.at("/get-ip").get(|req: Request<()>| async move {
        let ip = req.remote().unwrap().to_string();
        let response_body = json!({ "data": { "ip": ip } });
        Ok(response_body)
    });
    app.listen("127.0.0.1:9006").await?;
    Ok(())
}
