use crate::handlers::comment::{create_comment, get_post_comments};
use crate::handlers::media::upload_media;
use crate::handlers::post::{create_post, get_posts_by_space};
use crate::handlers::space::{create_space, get_space, join_space};
use crate::handlers::user::{get_users, login_user, register_user};
use crate::handlers::vote::{vote_comment, vote_post};
use crate::state::AppState;
use axum::Router;
use axum::routing::{get, post};
use tower_http::services::ServeDir;
use crate::handlers::admin::{kick_user, unban_user};

pub fn build_router(state: AppState) -> Router {
    Router::new()
        .route("/users", get(get_users))
        .nest("/admin", admin_routes())
        .nest("/auth", auth_routes())
        .nest("/spaces", space_routes())
        .nest("/posts", post_routes())
        .nest("/comments", comment_routes())
        .nest("/media", media_routes())
        .nest_service("/uploads", ServeDir::new("uploads"))
        .with_state(state)
}

fn admin_routes() -> Router<AppState> {
    Router::new()
        .route("/users/{target_user_id}/kick", post(kick_user))
        .route("/users/{target_user_id}/unban", post(unban_user))
}

fn auth_routes() -> Router<AppState> {
    Router::new()
        .route("/register", post(register_user))
        .route("/login", post(login_user))
}

fn space_routes() -> Router<AppState> {
    Router::new()
        .route("/", post(create_space).get(get_space))
        .route("/{space_id}/posts", get(get_posts_by_space))
        .route("/{space_id}/join", post(join_space))
}

fn post_routes() -> Router<AppState> {
    Router::new()
        .route("/", post(create_post))
        .route("/{post_id}/vote", post(vote_post))
        .route(
            "/{post_id}/comments",
            post(create_comment).get(get_post_comments),
        )
}

fn comment_routes() -> Router<AppState> {
    Router::new().route("/{comment_id}/vote", post(vote_comment))
}

fn media_routes() -> Router<AppState> {
    Router::new().route("/upload", post(upload_media))
}
