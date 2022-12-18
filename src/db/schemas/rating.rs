use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Rating {
   pub doctor_id: u32,
   pub rate: u8
}