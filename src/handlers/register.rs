use actix_web::{post, web, HttpResponse};
use serde::Deserialize;

use crate::{
    app_state::AppState,
    services::auth,
};

#[derive(Deserialize)]
pub struct RegisterRequest {
    pub login: String,
    pub password: String,
}

#[post("/login")]
pub async fn login(
    state: web::Data<AppState>,
    data: web::Json<RegisterRequest>,
) -> HttpResponse {
    println!("Login attempt for user: {}", data.login);
    let user = auth::find_user_by_login(&state.db, &data.login).await;
    println!("User found: {:?}", user);
    if user.is_none() {
        return HttpResponse::Unauthorized().body("User not found");
    }
    HttpResponse::Ok().body("Login successful")
}
