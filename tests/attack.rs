use pts_bomber_core::phone::{Phone, PhoneRu};
use pts_bomber_core::services::Services;
use pts_bomber_core::{attack::*, services::VictimRu};

#[tokio::test]
async fn send_first_ru() {
    let services = VictimRu {
        phone: PhoneRu::new("+79123456789").unwrap(),
        email: "".to_string(),
        name: "".to_string(),
        surname: "".to_string(),
    }
    .services();

    let first = services.into_iter().next().unwrap();
    send_single(first).await;
}
