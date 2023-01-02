use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Medicine {
    pub id: mongodb::bson::oid::ObjectId,
    pub name: String,
    pub amount: u32,
}
