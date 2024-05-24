use axum::{ Router};
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::routing::{get, post};
use axum_auth::AuthBasic;
use log::{info, warn};

use crate::{hash, RoadworkServerData};

pub(crate) fn user_routes() -> Router<RoadworkServerData> {
    Router::new()
        .route("/change_password", post(change_password))
        .route("/check/:bcrypted/:password", get(check))
        .route("/salt/:password", get(salt))
}

async fn change_password(AuthBasic((username, password)): AuthBasic,
                         State(state): State<RoadworkServerData>,
                         new_password: String) -> Result<StatusCode, &'static str> {
    info!("change_password username={}", username);
    if new_password.len() < 8 {
        warn!("Password is too short");
        return Err("Password is too short");
    }
    if password.is_none() {
        warn!("Password is missing");
        return Err("Password is missing");
    }
    let password = password.unwrap();
    if state.admin_service.get_user(&username, &password).await.is_some() {
        if state.admin_service.change_password(&username, &new_password).await.is_ok() {
            return Ok(StatusCode::NO_CONTENT);
        }
    }
    Err("Password not changed")
}

pub(crate) async fn check(Path((bcrypted, password)): Path<(String, String)>) -> &'static str {
    info!("check XXXXXXX");
    let result = hash::check(bcrypted.as_str(), &password);
    if result {
        "Password is correct"
    } else {
        "Password is incorrect"
    }
}

pub(crate) async fn salt(Path(password): Path<String>) -> String {
    info!("Salt XXXXXXX");
    let salted_password = hash::salt(&password);
    let response = format!("Bcrypt {} -> {}", password, salted_password);
    info!("Salt XXXXXXX -> {}", salted_password);
    response
}

