use axum::Router;
use axum::routing::{get, post};
use crate::handlers::user::{get_users, login_user, register_user};
use crate::state::AppState;

pub fn build_router(state: AppState) -> Router {
    Router::new()
        .route("/users", get(get_users))
        .route("/auth/register", post(register_user))
        .route("/auth/login", post(login_user))
        .with_state(state)
}