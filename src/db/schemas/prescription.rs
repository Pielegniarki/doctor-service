use mongodb::bson::oid::ObjectId;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Prescription {
    pub doctor_id: ObjectId,
    pub description: String,
    pub medicine: Vec<super::Medicine>,
}