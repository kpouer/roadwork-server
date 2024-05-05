use axum::{Json, Router};
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
    if let Some(password) = password {
        if let Some(user) = state.user_repository.find_user(&username).await {
            if user.is_valid(password) {
                if state.admin_service.change_password(username.as_str(), &new_password).await {
                    return Ok(StatusCode::NO_CONTENT);
                }
            } else {
                warn!("Password is incorrect");
                return Err("Password is incorrect");
            }
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

