use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Notification {
   pub doctor_id: u32,
   pub notification_message: String
}