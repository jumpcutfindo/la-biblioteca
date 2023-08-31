use std::net::SocketAddr;

use axum::extract::State;

use crate::database::setup_db;

mod app;
mod catalog;
mod database;
mod error;
mod library;
mod users;

#[tokio::main]
async fn main() {
    // Initialize tracing
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    let database_pool = setup_db(String::from("library.db")).unwrap();
    let state = app::create_new_state(database_pool);

    let app = app::create_app(State(state));

    // Run app using hyper, listens on port 3000
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::debug!("la-biblioteca server listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
