use std::sync::Arc;

use axum::{routing::{delete, get, post, put}, Router};
use tokio::sync::Mutex;

use crate::{app::user_services::UserService, infra::axum::controllers::user_controller::{self, UserController}};

pub fn user_routes(user_service: Arc<Mutex<UserService>>) -> Router {
    let controller = Arc::new(UserController::new(user_service));

    Router::new()
        .route("/users", post(user_controller::create_user))
        .route("/users/:id", get(user_controller::get_user))
        .route("/users", get(user_controller::list_users)) // Fix duplicate route
        .route("/users/:id", put(user_controller::update_user))
        .route("/users/:id", delete(user_controller::delete_user))
        .with_state(controller)
}
