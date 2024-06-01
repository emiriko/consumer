use bambangshop_receiver::Result;
use rocket::serde::json::Json;

use crate::{model::subscriber::SubscriberRequest, service::notification::NotificationService};

#[get("/susbscribe/<product_type>")]
pub fn subscribe(product_type: &str) -> Result<Json<SubscriberRequest>> {
    return match NotificationService::subscribe(product_type) {
        Ok(f) => Ok(Json::from(f)),
        Err(e) => Err(e)
    }
}