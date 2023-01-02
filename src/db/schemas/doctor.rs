use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Doctor {
    #[serde(skip)]
    pub _id: mongodb::bson::oid::ObjectId,
    pub name: String,
}