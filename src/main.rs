pub mod graph;
pub mod utils;
pub mod services;
pub mod models;

use std::env;
use graph::mutations::MutationRoot;
use graph::queries::Query;
use graph::subscriptions::SubscriptionRoot;
use tracing_subscriber::EnvFilter;
use aws_sdk_dynamodb::Client;
use async_graphql::Schema;
use poem::{
    error::NotFoundError,
    get, handler,
    http::StatusCode,
    listener::TcpListener,
    middleware::{Cors, Tracing},
    web::{Data, Html},
    EndpointExt, IntoResponse, Route, Server,
};
use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};
use async_graphql_poem::{GraphQLRequest, GraphQLResponse};

pub type PlatformSchema = Schema<Query, MutationRoot, SubscriptionRoot>;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    if env::var_os("RUST_LOG").is_none() {
        env::set_var("RUST_LOG", "warn");
    }

    //Setup logging & tracing
    tracing_subscriber::fmt()
        .json()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    let config = aws_config::load_from_env().await;    
    let client = Client::new(&config);
    
    let schema = Schema::build(Query::default(), MutationRoot::default(), SubscriptionRoot)
        .data(client)
        .finish(); 

    let app = Route::new()
        .at("/", get(graphql_playground).post(graphql_handler))
        .catch_error(|_err: NotFoundError| async move {
            poem::Response::builder()
                .status(StatusCode::NOT_FOUND)
                .body("route not found.");
        })
        .data(schema)
        .with(Tracing);

    let port = env::var("PORT");
    let port = match port {
        Ok(port) => port,
        Err(_) => "8000".to_string(),
    };

    Server::new(TcpListener::bind("0.0.0.0:".to_owned() + port.as_str()))
        .name("grafbase-api")
        .run(app.with(Cors::new()))
        .await    

}

#[handler]
fn graphql_playground() -> impl IntoResponse {
    Html(playground_source(
        GraphQLPlaygroundConfig::new("/").subscription_endpoint("/ws"),
    ))
}

#[handler]
async fn graphql_handler(
    schema: Data<&PlatformSchema>,
    req: GraphQLRequest,
) -> GraphQLResponse {
    schema.execute(req.0).await.into()
}
