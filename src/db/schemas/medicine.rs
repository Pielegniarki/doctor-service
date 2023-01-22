use mongodb::bson::oid::ObjectId;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Medicine {
    pub _id: ObjectId,
    pub name: String,
    pub amount: u32,
}
