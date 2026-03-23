use rocket::serde::json::Json;
use bambangshop::{Result, compose_error_response};
use crate::model::notification::Notification;
use crate::service::notification::NotificationService;

#[post("/subscribe/<product_type>")]
pub fn subscribe(product_type: &str) -> Json<&str> {
    NotificationService::subscribe(product_type);
    Json("Subscribed")
}

#[post("/unsubscribe/<product_type>")]
pub fn unsubscribe(product_type: &str) -> Json<&str> {
    NotificationService::unsubscribe(product_type);
    Json("Unsubscribed")
}

#[post("/receive", data = "<payload>")]
pub fn receive(payload: Json<Notification>) -> Result<Notification> {
    let result = NotificationService::receive_notification(payload.into_inner());
    match result {
        Ok(notification) => Ok(Json(notification)),
        Err(e) => Err(compose_error_response(
            rocket::http::Status::BadRequest,
            String::from(e)
        )),
    }
}
