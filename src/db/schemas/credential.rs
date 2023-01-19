use mongodb::bson::oid::ObjectId;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Credential {
    pub doctor_id: ObjectId,
    pub email: String,
    pub password: String,
}
