use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::routing::{get, post};
use axum::{Json, Router};
use axum_auth::AuthBasic;
use log::{info, warn};

use crate::model::user::User;
use crate::{hash, RoadworkServerData};

pub(crate) fn user_routes() -> Router<RoadworkServerData> {
    Router::new()
        .route("/change_password", post(change_password))
        .route("/info", get(get_user))
        .route("/check/:bcrypted/:password", get(check))
        .route("/salt", post(salt))
        .route("/test_connection/:teamname", get(test_connection))
}

async fn test_connection(
    AuthBasic((username, password)): AuthBasic,
    State(state): State<RoadworkServerData>,
    Path(teamname): Path<String>,
) -> String {
    info!("test_connection user={}, teamname={}", username, teamname);
    let password: String = password.unwrap_or_else(|| "".to_string());
    if let Some(user) = state.admin_service.get_user(&username, &password).await {
        if user.teams.contains(&teamname) {
            return "OK".to_string();
        }
        return format!("User {} is not in team {}", username, teamname);
    }

    "User is invalid".to_string()
}

async fn change_password(
    AuthBasic((username, password)): AuthBasic,
    State(state): State<RoadworkServerData>,
    new_password: String,
) -> Result<StatusCode, &'static str> {
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
    if state
        .admin_service
        .get_user(&username, &password)
        .await
        .is_some()
        && state
            .admin_service
            .change_password(&username, &new_password)
            .await
            .is_ok()
    {
        return Ok(StatusCode::NO_CONTENT);
    }
    Err("Password not changed")
}

async fn get_user(
    AuthBasic((username, password)): AuthBasic,
    State(state): State<RoadworkServerData>,
) -> Result<Json<User>, StatusCode> {
    info!("get_user username={}", username);
    let password: String = password.unwrap_or_else(|| "".to_string());
    if let Some(user) = state.admin_service.get_user(&username, &password).await {
        let mut web_user = user.clone();
        web_user.password_hash = "?????".to_string();
        info!("get_user -> {:?}", web_user);
        Ok(Json(web_user))
    } else {
        warn!("get_user user is invalid");
        Err(StatusCode::UNAUTHORIZED)
    }
}

async fn check(Path((bcrypted, password)): Path<(String, String)>) -> &'static str {
    info!("check XXXXXXX");
    let result = hash::check(bcrypted.as_str(), &password);
    if result {
        "Password is correct"
    } else {
        "Password is incorrect"
    }
}

async fn salt(password: String) -> String {
    info!("Salt XXXXXXX");
    let salted_password = hash::salt(&password);
    let response = format!("Bcrypt {} -> {}", password, salted_password);
    info!("Salt XXXXXXX -> {}", salted_password);
    response
}
