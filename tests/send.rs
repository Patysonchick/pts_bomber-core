use pts_bomber_core::phone::{Phone, PhoneRu};
use pts_bomber_core::{send::*, services::*};
use std::sync::atomic::AtomicBool;
use std::sync::Arc;

#[tokio::test]
async fn send_first_ru() {
    let victim = VictimRu {
        phone: PhoneRu::new("+79123456789").unwrap(),
        email: "".to_string(),
        name: "".to_string(),
        surname: "".to_string(),
    };

    let stop_flag = Arc::new(AtomicBool::new(false));

    let sms = victim.sms().into_iter().next().unwrap();
    single(sms, stop_flag.clone()).await;

    let call = victim.sms().into_iter().next().unwrap();
    single(call, stop_flag.clone()).await;

    let services = victim.services().into_iter().next().unwrap();
    single(services, stop_flag.clone()).await;
}

#[tokio::test]
async fn send_ru() {
    let victim = VictimRu {
        phone: PhoneRu::new("+79123456789").unwrap(),
        email: "".to_string(),
        name: "".to_string(),
        surname: "".to_string(),
    };

    let stop_flag = Arc::new(AtomicBool::new(false));
    victim.send(1, stop_flag).await;
}
