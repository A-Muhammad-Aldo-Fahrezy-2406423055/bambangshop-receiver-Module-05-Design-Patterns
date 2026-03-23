use bambangshop::Result;
use crate::model::notification::Notification;
use crate::repository::notification::NotificationRepository;
use crate::APP_CONFIG;

lazy_static::lazy_static! {
    static ref REQWEST_CLIENT: reqwest::Client = reqwest::Client::new();
}

pub struct NotificationService;

impl NotificationService {

    async fn subscribe_request(product_type: &str) {
        let publisher_url = APP_CONFIG.get_publisher_root_url();
        let target_url = format!("{}/notification/subscribe/{}", publisher_url, product_type);
        let my_url = format!("{}/receive", APP_CONFIG.get_instance_root_url());
        
        let payload = crate::model::subscriber::SubscriberRequest {
            name: APP_CONFIG.get_instance_name().clone(),
            url: my_url,
        };

        match REQWEST_CLIENT.post(&target_url).json(&payload).send().await {
            Ok(res) => println!("Successfully subscribed to {}: {}", product_type, res.status()),
            Err(e) => println!("Failed to subscribe to {}: {}", product_type, e),
        }
    }

    pub fn subscribe(product_type: &str) {
        let pt = String::from(product_type);
        tokio::spawn(async move {
            Self::subscribe_request(&pt).await;
        });
    }

    async fn unsubscribe_request(product_type: &str) {
        let publisher_url = APP_CONFIG.get_publisher_root_url();
        let target_url = format!("{}/notification/unsubscribe/{}?url={}/receive", 
            publisher_url, 
            product_type,
            APP_CONFIG.get_instance_root_url()
        );

        match REQWEST_CLIENT.post(&target_url).send().await {
            Ok(res) => println!("Successfully unsubscribed from {}: {}", product_type, res.status()),
            Err(e) => println!("Failed to unsubscribe from {}: {}", product_type, e),
        }
    }

    pub fn unsubscribe(product_type: &str) {
        let pt = String::from(product_type);
        tokio::spawn(async move {
            Self::unsubscribe_request(&pt).await;
        });
    }
}
