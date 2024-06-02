#[macro_use] extern crate rocket;

mod models;
mod routes;
mod schema;
use std::sync::{Arc, Mutex};
use routes::{one,graph};
use schema::schema::build_schema;
use models::user_model::Users;

#[launch]
fn rocket()-> _ {
    let users = Arc::new(Mutex::new(Users::new())); // Initialize users data

    rocket::build()
    .manage(build_schema(users.clone()))
    .mount("/",routes![one::get_live])
    .mount("/graph",graph::routes())
}
