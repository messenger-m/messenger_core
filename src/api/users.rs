use axum::{Router, routing::post};

// use crate::handlers::login::{login_handler, AppState};
#[derive(Clone)]
pub struct AppState {
    pub pg: Client,
}
pub fn create_router(state: AppState) -> Router {
    Router::new()
        .route("/login", post(login_handler))
        .with_state(state)
}