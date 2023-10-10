use warp::Filter;
use serde_json::json;

#[tokio::main]
async fn main() {
    // Создаем фильтр для маршрута /get-ip
    let get_ip = warp::path("get-ip")
        .and(warp::addr::remote())  // Извлекаем IP-адрес клиента
        .map(|ip: Option<std::net::SocketAddr>| {
            // Преобразуем IP-адрес в строку JSON и отправляем его обратно
            let ip_str = ip.map(|sock_addr| sock_addr.ip().to_string()).unwrap_or_default();
            warp::reply::json(&json!({ "data": { "ip": ip_str }}))
        });

    // Запускаем сервер на порту 9007
    warp::serve(get_ip)
        .run(([127, 0, 0, 1], 9007))
        .await;
}

