use rocket::serde::json::Json;
use crate::models::general::Record;

#[get("/")]
pub fn get_live()-> Json<Record>{
    Json(Record{
        id:1,
        name:"Router is alive".to_string()
    })
}