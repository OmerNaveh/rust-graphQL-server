use rocket::State;
use rocket::response::content::RawHtml;
use async_graphql_rocket::{GraphQLRequest, GraphQLResponse};
use crate::schema::schema::AppSchema;


#[get("/")]
fn graphql_playground() -> RawHtml<&'static str> {
    let html_content = include_str!("../../static/playground.html");
    RawHtml(html_content)
}

#[post("/graphql", data = "<request>")]
async fn graphql_handler(schema: &State<AppSchema>, request: GraphQLRequest) -> GraphQLResponse {
   request.execute(schema.inner()).await
}

pub fn routes() -> Vec<rocket::Route> {
    routes![graphql_playground , graphql_handler]
}