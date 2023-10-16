use std::collections::HashMap;
use std::time::Instant;
use serde_json::Value;
use reqwest;

fn main() {
    let mut map: HashMap<String, HashMap<String, Value>> = [
        ("flask".to_string(), [("value".to_string(), Value::from(0.0)), ("url".to_string(), Value::from("http://127.0.0.1:9002/get-ip"))].iter().cloned().collect()),
        ("django".to_string(), [("value".to_string(), Value::from(0.0)), ("url".to_string(), Value::from("http://127.0.0.1:9001/get-ip"))].iter().cloned().collect()),
        ("fastapi".to_string(), [("value".to_string(), Value::from(0.0)), ("url".to_string(), Value::from("http://127.0.0.1:8000/get-ip"))].iter().cloned().collect()),
        ("nestjs".to_string(), [("value".to_string(), Value::from(0.0)), ("url".to_string(), Value::from("http://127.0.0.1:9003/get-ip"))].iter().cloned().collect()),
        ("express".to_string(), [("value".to_string(), Value::from(0.0)), ("url".to_string(), Value::from("http://127.0.0.1:3000/get-ip"))].iter().cloned().collect()),
        ("actix_web".to_string(), [("value".to_string(), Value::from(0.0)), ("url".to_string(), Value::from("http://127.0.0.1:9004/get-ip"))].iter().cloned().collect()),
        ("rocket".to_string(), [("value".to_string(), Value::from(0.0)), ("url".to_string(), Value::from("http://127.0.0.1:9005/get-ip"))].iter().cloned().collect()),
        ("tide".to_string(), [("value".to_string(), Value::from(0.0)), ("url".to_string(), Value::from("http://127.0.0.1:9006/get-ip"))].iter().cloned().collect()),
        ("warp".to_string(), [("value".to_string(), Value::from(0.0)), ("url".to_string(), Value::from("http://127.0.0.1:9007/get-ip"))].iter().cloned().collect()),
        ("cherypi".to_string(), [("value".to_string(), Value::from(0.0)), ("url".to_string(), Value::from("http://127.0.0.1:9008/get-ip"))].iter().cloned().collect()),
        ("bottle".to_string(), [("value".to_string(), Value::from(0.0)), ("url".to_string(), Value::from("http://127.0.0.1:9009/get-ip"))].iter().cloned().collect()),
        ("tornado".to_string(), [("value".to_string(), Value::from(0.0)), ("url".to_string(), Value::from("http://127.0.0.1:9010/get-ip"))].iter().cloned().collect()),
        ("pyramid".to_string(), [("value".to_string(), Value::from(0.0)), ("url".to_string(), Value::from("http://127.0.0.1:9011/get-ip"))].iter().cloned().collect()),
        ("web.py".to_string(), [("value".to_string(), Value::from(0.0)), ("url".to_string(), Value::from("http://127.0.0.1:9014/get-ip"))].iter().cloned().collect()),
        ("echo".to_string(), [("value".to_string(), Value::from(0.0)), ("url".to_string(), Value::from("http://127.0.0.1:9015/get-ip"))].iter().cloned().collect()),
        ("fiber".to_string(), [("value".to_string(), Value::from(0.0)), ("url".to_string(), Value::from("http://127.0.0.1:9016/get-ip"))].iter().cloned().collect()),
        ("fasthttp".to_string(), [("value".to_string(), Value::from(0.0)), ("url".to_string(), Value::from("http://127.0.0.1:9017/get-ip"))].iter().cloned().collect()),
        ("gorilla".to_string(), [("value".to_string(), Value::from(0.0)), ("url".to_string(), Value::from("http://127.0.0.1:9018/get-ip"))].iter().cloned().collect()),
        ("asp.net".to_string(), [("value".to_string(), Value::from(0.0)), ("url".to_string(), Value::from("http://127.0.0.1:5090/GetIp/get-ip"))].iter().cloned().collect())
    ]
    .iter()
    .cloned()
    .collect();

    for key in map.keys().cloned().collect::<Vec<String>>() {
        for _ in 0..1000 {
            let url = map.get(&key).unwrap().get("url").unwrap().as_str().unwrap();
            let elapsed = get_time(url);
            let old_value: f64 = map.get(&key).unwrap().get("value").unwrap().as_f64().unwrap();
            let new_value = old_value + elapsed;
            map.get_mut(&key).unwrap().insert("value".to_string(), Value::from(new_value));
        }
    }

    let mut keys: Vec<&String> = map.keys().collect();
    keys.sort_by(|a, b| {
        let a_value: f64 = map.get(*a).unwrap().get("value").unwrap().as_f64().unwrap();
        let b_value: f64 = map.get(*b).unwrap().get("value").unwrap().as_f64().unwrap();
        a_value.partial_cmp(&b_value).unwrap()
    });

    println!("Список фреймворков и их скорости:");

    for key in &keys {
        let value: f64 = map.get(*key).unwrap().get("value").unwrap().as_f64().unwrap();
        println!("{}: {}", key, value);
    }
}

fn get_time(url: &str) -> f64 {
    let start_time = Instant::now();
    let _ = reqwest::blocking::get(url); // Обработай ошибки здесь как хочешь
    let end_time = Instant::now();
    let elapsed = end_time.duration_since(start_time).as_secs_f64();
    elapsed
}
