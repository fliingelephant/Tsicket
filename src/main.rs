//! test demo of actix-web
extern crate juniper;

use actix_web::{App, Error,
                HttpResponse, HttpServer,
                middleware, web};
use juniper::http::graphiql::graphiql_source;
use juniper::http::GraphQLRequest;

mod schema;

fn main() {
    println!("Hello, world!");
}
