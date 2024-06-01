use lazy_static::lazy_static;
use rocket::tokio::sync::RwLock;

use crate::model::notification::Notification;

// Singleton of Database
lazy_static! {
    static ref NOTIFICATIONS: RwLock<Vec<Notification>> = RwLock::new(vec![]);
}

pub struct NotificationRepository;

impl NotificationRepository {
    
}