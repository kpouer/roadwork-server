use axum::http::StatusCode;
use axum::routing::get;
use axum::Router;
use log::info;
use crate::error::Error;
use crate::router::admin::admin_routes;
use crate::router::roadwork::roadwork_routes;
use crate::router::user::user_routes;
use crate::service::user::AdminService;
use crate::service::user_repository::UserRepository;

mod hash;
mod router;
mod service;
mod error;

#[derive(Clone)]
pub(crate) struct RoadworkServerData {
    user_repository: UserRepository,
    admin_service: AdminService,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    env_logger::init();
    info!("Starting Roadwork server");
    let user_repository = UserRepository::new().await?;
    let admin_service = AdminService::new(user_repository.clone()).await;
    let roadwork_server_data = RoadworkServerData {
        user_repository,
        admin_service,
    };
    let app = Router::new()
        .route("/info", get(|| async { "Roadwork server by kpouer" }))
        .route("/ping", get(|| async { "pong" }))
        .nest("/admin", admin_routes())
        .nest("/user", user_routes())
        .nest("/roadwork", roadwork_routes())
        .with_state(roadwork_server_data)
        .fallback(|| async { (StatusCode::NOT_FOUND, "Not Found") });
    let addr = "0.0.0.0:8080";
    let listener = tokio::net::TcpListener::bind(addr).await?;
    info!("Listen on {addr}");
    axum::serve(listener, app).await?;
    Ok(())
}
