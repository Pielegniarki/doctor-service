use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Doctor {
    pub name: String
}