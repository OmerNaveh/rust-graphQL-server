use rocket::serde::{Serialize, Deserialize};
use async_graphql::SimpleObject;

#[derive(Debug, Serialize, Deserialize,Clone,SimpleObject)]
pub struct User {
    pub id: u32,
    pub first_name: String,
    pub last_name:String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Users {
    pub users:  Vec<User>,
}

impl Users {
    pub fn new() -> Self {
        Users { users: Vec::new() }
    }
}


#[derive(Debug, Serialize, Deserialize)]
pub struct Post {
    pub id: u32,
    pub user_id: u32,
    pub content:String,
}