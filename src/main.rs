use axum::Router;
use log::info;

use crate::router::admin::admin_routes;
use crate::service::admin_service::AdminService;
use crate::service::user_repository::UserRepository;

mod hash;
mod router;
mod model;
mod service;

#[derive(Clone)]
pub(crate) struct RoadworkServerData {
    user_repository: UserRepository,
    admin_service: AdminService,
}

#[tokio::main]
async fn main() {
    env_logger::init();
    log::info!("Starting Roadwork server");
    let user_repository = UserRepository::new().await.unwrap();
    let admin_service = AdminService::new(user_repository.clone()).await;
    let roadwork_server_data = RoadworkServerData {
        user_repository,
        admin_service,
    };
    let app = Router::new()
        .nest("/admin", admin_routes())
        .with_state(roadwork_server_data)
        ;
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    info!("Listen on 0.0.0.0:8080");
    axum::serve(listener, app).await.unwrap();
}
