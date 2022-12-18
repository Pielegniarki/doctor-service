use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Medicine {
    pub id: u32,
    pub name: String,
    pub amount: u32,
}
