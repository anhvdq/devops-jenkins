use std::sync::Arc;

use crate::repository::user_repository::UserRepository;
use crate::service::user_service::UserService;
use crate::{
    config::pg_database::PgDatabase, repository::user_repository::UserRepositoryImpl,
    service::user_service::UserServiceImpl,
};

use axum::{Router, routing::IntoMakeService};
use tower_http::trace::TraceLayer;

use super::user;

pub fn routes(db_conn: Arc<PgDatabase>) -> IntoMakeService<Router> {
    // Initialize user service
    let user_repository: UserRepository = Arc::new(UserRepositoryImpl::new(Arc::clone(&db_conn)));
    let user_service: UserService = Arc::new(UserServiceImpl::new(Arc::clone(&user_repository)));

    let public = Router::new()
        .merge(user::routes(Arc::clone(&user_service)))
        .merge(Router::new().route(
            "/health-check",
            axum::routing::get(|| async { format!("App version: {}", env!("CARGO_PKG_VERSION")) }),
        ));

    let app_router = Router::new().merge(public);
    app_router
        .layer(TraceLayer::new_for_http()) // Enable logging
        .into_make_service()
}
