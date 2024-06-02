use std::sync::{Arc, Mutex};

use async_graphql::{Context, Object, Schema, EmptySubscription};
use crate::models::user_model::{User,Users};

pub struct Query;

#[Object]
impl Query {
    async fn hello(&self, _ctx: &Context<'_>) -> &str {
        "Hello, world!"
    }

    async fn goodbye(&self , _ctx:&Context<'_>) ->&str{
        "Goodbye, World!"
    }

   async fn find_user(&self, ctx: &Context<'_>, id: u32) -> Option<User> {
    let users = match ctx.data::<Arc<Mutex<Users>>>() {
        Ok(users) => users,
        Err(_err) => {
            eprintln!("Failed to retrieve users data");
            return None;
        }
    };
    
    let locked_users = match users.lock() {
        Ok(users)=>users,
        Err(_e) => return None
    };
    let user = locked_users.users.iter().find(|user| user.id == id).cloned();
    user
    }
}


pub struct Mutation;

#[Object]
impl Mutation{
    async fn create_user(&self, ctx: &Context<'_>, first_name: String, last_name: String) -> Option<User> {
        // Create a new user and add it to the context
        let users = ctx.data::<Arc<Mutex<Users>>>().expect("Context not found");
        let mut users = users.lock().expect("Failed to acquire lock");
        let id = (users.users.len() as u32) + 1;
        let user = User {
            id,
            first_name,
            last_name,
        };
        users.users.push(user.clone());
        println!("{:?}",users.users);
        Some(user)
    }
}


pub type AppSchema = Schema<Query, Mutation, EmptySubscription>;

pub fn build_schema(users: Arc<Mutex<Users>>) -> AppSchema {
    Schema::build(Query, Mutation, EmptySubscription).data(users).finish()
}
