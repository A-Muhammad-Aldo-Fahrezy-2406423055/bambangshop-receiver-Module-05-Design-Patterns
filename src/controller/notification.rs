use rocket::serde::json::Json;
use bambangshop::{Result, compose_error_response};
use crate::model::notification::Notification;
use crate::service::notification::NotificationService;

#[post("/subscribe/<product_type>")]
pub fn subscribe(product_type: &str) -> Json<&str> {
    NotificationService::subscribe(product_type);
    Json("Subscribed")
}
