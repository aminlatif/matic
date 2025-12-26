use std::sync::Arc;

use axum::{Router, middleware, routing::get};
use tower::ServiceBuilder;

use crate::{
    handlers::root_handler,
    routes::{auth::auth_routes, health::health_routes, user::user_routes},
    state::AppState,
};

pub mod auth;
pub mod health;
pub mod user;

pub fn get_router(app_state: Arc<AppState>) -> Router {
    Router::new()
        .route("/", get(root_handler))
        .nest("/auth", auth_routes(&app_state))
        .nest("/health", health_routes(&app_state))
        .nest("/user", user_routes(&app_state))
        .with_state(Arc::clone(&app_state))
        // .layer(
        //     ServiceBuilder::new()
        //         .layer(middleware::from_fn_with_state(
        //             Arc::clone(&app_state),
        //             crate::middleware::authenticate::authenticate,
        //         ))
        //         .layer(middleware::from_fn_with_state(
        //             Arc::clone(&app_state),
        //             crate::middleware::authorize::authorize,
        //         )),
        // )

    // .nest("/api", api_routes())
    // .layer(
    //     ServiceBuilder::new()
    //         // Order matters - from outer to inner
    //         .layer(HandleErrorLayer::new(handle_error))
    //         .layer(TraceLayer::new_for_http())
    //         .layer(CompressionLayer::new())
    //         .layer(TimeoutLayer::new(Duration::from_secs(30)))
    //         .layer(CorsLayer::permissive()) // Configure properly in prod
    //         .layer(
    //             // Rate limiting
    //             governor::GovernorLayer {
    //                 config: governor::GovernorConfigBuilder::default()
    //                     .per_second(60)
    //                     .burst_size(100)
    //                     .finish()
    //                     .unwrap(),
    //             },
    //         )
    //         .layer(axum_login::AuthManagerLayer::new(
    //             |user_id| async move {
    //                 // Fetch user from DB
    //             },
    //             &secret,
    //             cookie::Key::from(&[0; 64]), // Use proper key
    //         )),
    // );

    // Ok(app)
}
