use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Specialty {
    pub _id: mongodb::bson::oid::ObjectId,
    pub name: String,
}