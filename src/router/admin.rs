use axum::{Json, Router};
use axum::extract::State;
use axum::http::StatusCode;
use axum::routing::get;
use axum_auth::AuthBasic;
use log::{info, warn};

use crate::RoadworkServerData;
use crate::service::user::AdminService;

pub(crate) fn admin_routes() -> Router<RoadworkServerData> {
    Router::new()
        .route("/teams", get(list_teams))
        .route("/users", get(list_users))
}

pub(crate) async fn list_teams(AuthBasic((username, password)): AuthBasic,
                               State(state): State<RoadworkServerData>) -> Result<Json<Vec<String>>, StatusCode> {
    info!("list_teams");
    check_admin(&state.admin_service, &username, &password).await?;
    let teams = state.user_repository.list_teams().await;
    info!("list_teams -> {:?}", teams);
    Ok(Json(teams))
}

pub(crate) async fn list_users(State(state): State<RoadworkServerData>) -> Result<Json<Vec<String>>, StatusCode> {
    info!("list_users");
    check_admin(&state.admin_service, &"admin".to_string(), &None).await?;
    let user_names = state.user_repository.list_users().await;
    info!("list_users -> {:?}", user_names);
    Ok(Json(user_names))
}

async fn check_admin(admin_service: &AdminService, username: &String, password: &Option<String>) -> Result<(), StatusCode> {
    if let Some(password) = password {
        if admin_service.is_admin(username, password).await {
            return Ok(());
        }
    }
    warn!("User {} is not an admin", username);
    return Err(StatusCode::UNAUTHORIZED);
}