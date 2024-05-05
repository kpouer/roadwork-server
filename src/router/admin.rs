use axum::{Json, Router};
use axum::extract::State;
use axum::http::StatusCode;
use axum::routing::get;
use axum_auth::AuthBasic;
use log::{info, warn};

use crate::RoadworkServerData;

pub(crate) fn admin_routes() -> Router<RoadworkServerData> {
    Router::new()
        .route("/teams", get(list_teams))
        .route("/users", get(list_users))
}

pub(crate) async fn list_teams(AuthBasic((username, password)): AuthBasic,
                               State(state): State<RoadworkServerData>) -> Result<Json<Vec<String>>, StatusCode> {
    if !state.admin_service.is_admin(&username, password).await {
        warn!("User {} is not admin", username);
        return Err(StatusCode::UNAUTHORIZED);
    }
    info!("list_teams");
    let teams = state.user_repository.list_teams().await;
    info!("list_teams -> {:?}", teams);
    Ok(Json(teams))
}

pub(crate) async fn list_users(State(state): State<RoadworkServerData>) -> Json<Vec<String>> {
    info!("list_users");
    let user_names = state.user_repository.list_users().await;
    info!("list_users -> {:?}", user_names);
    Json(user_names)
}

