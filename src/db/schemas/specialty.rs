use mongodb::bson::oid::ObjectId;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Specialty {
    #[serde(skip)]
    pub _id: ObjectId,
    pub name: String
}