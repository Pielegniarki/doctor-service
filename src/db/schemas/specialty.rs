use mongodb::bson::oid::ObjectId;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Specialty {
    pub _id: ObjectId,
    pub name: String
}