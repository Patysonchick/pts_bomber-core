use crate::services::{BodyType, Service};
use reqwest::{Client, Method};
pub async fn send_single(service: Service) {
    let client = Client::builder()
        .user_agent("Mozilla/5.0 (X11; Linux x86_64; rv:129.0) Gecko/20100101 Firefox/129.0")
        .default_headers(service.headers)
        .build()
        .expect("");

    let mut res;
    match service.method {
        Method::GET => res = client.get(service.url),
        Method::POST => res = client.post(service.url),
        _ => panic!("Unsupported method"),
    }
    match service.body_type {
        BodyType::JSON => res = res.json(&service.body),
        BodyType::Form => res = res.form(&service.body),
    }

    println!("Starting {}", service.name);
    let res = res.send().await.expect("");
    println!(
        "{} {}\n{}\n",
        service.name,
        res.status(),
        res.text().await.unwrap()
    );
}
