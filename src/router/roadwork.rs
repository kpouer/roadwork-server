use std::collections::HashMap;

use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::routing::post;
use axum::{Json, Router};
use axum_auth::AuthBasic;
use log::warn;

use crate::model::sync_data::SyncData;
use crate::service::data;
use crate::{info, RoadworkServerData};

pub(crate) fn roadwork_routes() -> Router<RoadworkServerData> {
    Router::new().route("/set_data/:team/:opendata_service", post(set_data))
}

pub(crate) async fn set_data(
    AuthBasic((username, password)): AuthBasic,
    State(state): State<RoadworkServerData>,
    Path((team, opendata_service)): Path<(String, String)>,
    Json(sync_data_list): Json<HashMap<String, SyncData>>,
) -> Result<Json<HashMap<String, SyncData>>, StatusCode> {
    let password: String = password.unwrap_or_else(|| "".to_string());
    return if state
        .admin_service
        .has_team(&username, &password, &team)
        .await
    {
        info!("set_data user={username} team={team} service={opendata_service}");
        let opendata_service = opendata_service
            .strip_suffix(".json")
            .unwrap_or(&opendata_service);

        let string_sync_data_map = data::set_data(team.as_str(), opendata_service, sync_data_list);
        Ok(Json(string_sync_data_map))
    } else {
        warn!("User {username} is not valid for team {team}");
        Err(StatusCode::UNAUTHORIZED)
    };
}
