use rocket::{get, post, serde::json::Json};
use crate::service::notification::NotificationService;
use crate::model::subscriber::SubscriberRequest;
use crate::model::notification::Notification;
use bambangshop_receiver::APP_CONFIG;

#[post("/subscribe/<product_type>")]
pub fn subscribe(product_type: &str) -> Json<SubscriberRequest> {
    NotificationService::subscribe(product_type);
    let my_url = format!("{}/receive", APP_CONFIG.get_instance_root_url());
    Json(SubscriberRequest {
        name: APP_CONFIG.get_instance_name().clone(),
        url: my_url,
    })
}

#[post("/unsubscribe/<product_type>")]
pub fn unsubscribe(product_type: &str) -> Json<SubscriberRequest> {
    NotificationService::unsubscribe(product_type);
    let my_url = format!("{}/receive", APP_CONFIG.get_instance_root_url());
    Json(SubscriberRequest {
        name: APP_CONFIG.get_instance_name().clone(),
        url: my_url,
    })
}

#[post("/receive", format = "json", data = "<notification>")]
pub fn receive(notification: Json<Notification>) -> Json<Notification> {
    let result = NotificationService::receive_notification(notification.into_inner());
    match result {
        Ok(n) => Json(n),
        Err(e) => panic!("{}", e)
    }
}

#[get("/list")]
pub fn list() -> Json<Vec<String>> {
    let messages = NotificationService::list_messages();
    Json(messages)
}
