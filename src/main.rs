#[macro_use]
extern crate rocket;

use juniper::{EmptyMutation, EmptySubscription};
use juniper_rocket;
use rocket::{response::content, State};

mod graphql_schema;

use crate::graphql_schema::{create_schema, QueryRoot, Schema};

/// the graphiql interface
#[get("/")]
fn graphiql() -> content::RawHtml<String> {
    juniper_rocket::graphiql_source("/graphql", None)
}

/// handles graphql queries where the query
/// is part of the request body
#[post("/graphql", data = "<request>")]
async fn post_graphql_handler(
    context: &State<()>,
    request: juniper_rocket::GraphQLRequest,
    schema: &State<Schema>,
) -> juniper_rocket::GraphQLResponse {
    request.execute(&schema, &context).await
}

#[get("/")]
fn index() -> &'static str {
    "Hello, world"
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .manage(())
        .manage(Schema::new(
            QueryRoot,
            EmptyMutation::new(),
            EmptySubscription::new(),
        ))
        .mount("/", routes![graphiql, post_graphql_handler])
}
