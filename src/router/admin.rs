use axum::{Json, Router};
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::routing::{delete, get, post};
use axum_auth::AuthBasic;
use log::{info, warn};
use crate::model::user::User;

use crate::RoadworkServerData;
use crate::service::user::AdminService;

pub(crate) fn admin_routes() -> Router<RoadworkServerData> {
    Router::new()
        .route("/teams", get(list_teams))
        .route("/team/:team_name", post(add_team))
        .route("/users", get(list_users))
        .route("/user", post(add_user))
        .route("/user/:user_name", delete(delete_user))
}

pub(crate) async fn list_teams(AuthBasic((username, password)): AuthBasic,
                               State(state): State<RoadworkServerData>) -> Result<Json<Vec<String>>, StatusCode> {
    info!("list_teams");
    check_admin(&state.admin_service, &username, &password).await?;
    let teams = state.user_repository.list_teams().await;
    info!("list_teams -> {:?}", teams);
    Ok(Json(teams))
}

pub(crate) async fn add_team(AuthBasic((username, password)): AuthBasic,
                             State(state): State<RoadworkServerData>,
                             Path(team_name): Path<String>) -> Result<&'static str, StatusCode> {
    info!("add_team {}", team_name);
    check_admin(&state.admin_service, &username, &password).await?;
    state.user_repository.insert_team(team_name.as_str()).await.map_or(Ok("KO"), |_| Ok("OK"))
}

pub(crate) async fn list_users(AuthBasic((username, password)): AuthBasic,
                               State(state): State<RoadworkServerData>) -> Result<Json<Vec<String>>, StatusCode> {
    info!("list_users");
    check_admin(&state.admin_service, &username, &password).await?;
    let user_names = state.user_repository.list_users().await;
    info!("list_users -> {:?}", user_names);
    Ok(Json(user_names))
}

pub(crate) async fn add_user(AuthBasic((username, password)): AuthBasic,
                             State(state): State<RoadworkServerData>,
                             Json(new_user): Json<User>) -> Result<&'static str, StatusCode> {
    info!("add_user {:?}", new_user);
    check_admin(&state.admin_service, &username, &password).await?;
    state.user_repository.insert_user(&new_user).await.map_or(Ok("KO"), |_| Ok("OK"))
}

async fn delete_user(AuthBasic((username, password)): AuthBasic,
                     State(state): State<RoadworkServerData>,
                     Path(removed_user): Path<String>) -> Result<&'static str, StatusCode> {
    info!("remove_user {}", removed_user);
    check_admin(&state.admin_service, &username, &password).await?;
    state.user_repository.delete_user(&removed_user).await.map_or(Ok("KO"), |_| Ok("OK"))
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