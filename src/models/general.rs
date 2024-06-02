use rocket::serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Record {
    pub id: u32,
    pub name: String,
}
