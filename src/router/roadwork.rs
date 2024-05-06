use std::collections::HashMap;

use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::{Json, Router};
use axum::routing::{ post};
use axum_auth::AuthBasic;
use log::warn;

use crate::{info, RoadworkServerData};
use crate::model::sync_data::SyncData;
use crate::service::data;

pub(crate) fn roadwork_routes() -> Router<RoadworkServerData> {
    Router::new()
        .route("/set_data/:team/:opendata_service", post(set_data))
}

pub(crate) async fn set_data(AuthBasic((username, password)): AuthBasic,
                             State(state): State<RoadworkServerData>,
                             Path((team, opendata_service)): Path<(String, String)>,
                             Json(sync_data_list): Json<HashMap<String, SyncData>>) -> Result<Json<HashMap<String, SyncData>>, StatusCode> {
    let password: String = password.unwrap_or_else(|| "".to_string());
    if state.admin_service.has_team(&username, &password, &team).await {
        info!("set_data user={} team={} service={}", username, team, opendata_service);
        let opendata_service = remove_suffix(&opendata_service, ".json");

        let string_sync_data_map = data::set_data(team.as_str(), opendata_service, sync_data_list);
        return Ok(Json(string_sync_data_map));
    } else {
        warn!("User {} is not valid for team", username);
        return Err(StatusCode::UNAUTHORIZED);
    }
}

fn remove_suffix<'a>(input: &'a str, suffix: &str) -> &'a str {
    if input.ends_with(suffix) {
        return &input[..(input.len() - suffix.len())];
    }
    input
}