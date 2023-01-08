use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Doctor {
    pub _id: mongodb::bson::oid::ObjectId,
    pub name: String,
    pub specialties: Vec<mongodb::bson::oid::ObjectId>
}