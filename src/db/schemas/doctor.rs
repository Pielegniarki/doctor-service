use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Doctor {
    pub login: String,
    pub password: String,
    pub name: String
}