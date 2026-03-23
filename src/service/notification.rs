use bambangshop::Result;
use crate::model::notification::Notification;
use crate::repository::notification::NotificationRepository;
use crate::APP_CONFIG;

lazy_static::lazy_static! {
    static ref REQWEST_CLIENT: reqwest::Client = reqwest::Client::new();
}

pub struct NotificationService;

impl NotificationService {
}
