use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Prescription {
    pub doctor: String,
    pub description: String,
    pub medicine: Vec<super::Medicine>,
}