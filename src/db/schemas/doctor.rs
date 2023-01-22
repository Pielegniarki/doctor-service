use mongodb::bson::oid::ObjectId;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Doctor {
    pub _id: ObjectId,
    pub name: String,
    pub specialties: Vec<ObjectId>
}