use std::thread;

use bambangshop_receiver::{compose_error_response, Result, APP_CONFIG, REQWEST_CLIENT};
use rocket::{http::Status, log, serde::json::to_string, tokio};

use crate::model::subscriber::SubscriberRequest;

pub struct NotificationService;

impl NotificationService {
    #[tokio::main]
    async fn subscribe_request(product_type: String) -> Result<SubscriberRequest> {
        let product_type_upper: String = product_type.to_uppercase();
        let product_type_str: &str = product_type_upper.as_str();
        let notification_receiver_url: String = format!("{}/receive", APP_CONFIG.get_instance_root_url());

        let payload: SubscriberRequest = SubscriberRequest { url: notification_receiver_url, name: APP_CONFIG.get_instance_name().to_string() };

        let request_url: String = format!("{}/notification/subscribe/{}", APP_CONFIG.get_publisher_root_url(), product_type_str);

        let request = REQWEST_CLIENT
            .post(request_url.clone())
            .header("Content-Type", "application/json")
            .header("Accept", "application/json")
            .body(to_string(&payload).unwrap())
            .send().await;

        log::warn_!("Sent subscribe request to: {}", request_url);

        return match request {
            Ok(f) => match f.json::<SubscriberRequest>().await {
                Ok(x) => Ok(x),
                Err(y) => Err(compose_error_response(
                    Status::NotAcceptable, y.to_string()))
            },
            Err(e) => Err(compose_error_response(
                Status::NotAcceptable, e.to_string()))
        }
    }
    pub fn subscribe(product_type: &str) -> Result<SubscriberRequest> {
        let product_type_clone = String::from(product_type);
        return thread::spawn(move || Self::subscribe_request(product_type_clone)).join().unwrap();
    }
}