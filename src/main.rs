use actix_web::{App, HttpServer, web};

mod handlers;
mod routes;
mod db;
mod models;
mod services;
mod app_state;

use app_state::AppState;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let db = db::postgres::connect().await;

    let state = web::Data::new(AppState { db });

    HttpServer::new(move || {
        App::new()
            .app_data(state.clone())
            .configure(routes::init_routes)
    })
    .bind(("127.0.0.1", 8000))?
    .run()
    .await
}
