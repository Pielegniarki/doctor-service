use mongodb::bson::oid::ObjectId;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Notification {
   pub doctor_id: ObjectId,
   pub notification_message: String
}