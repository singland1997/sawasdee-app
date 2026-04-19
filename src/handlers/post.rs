use crate::errors::AppError;
use crate::middleware::auth::AuthUser;
use crate::models::post::{CreatePostReq, Post};
use crate::state::AppState;
use anyhow::Result;
use axum::Json;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use uuid::Uuid;

pub async fn create_post(
    State(state): State<AppState>,
    user: AuthUser,
    Json(payload): Json<CreatePostReq>,
) -> Result<(StatusCode, Json<Post>), AppError> {
    let post = sqlx::query_as::<_, Post>(
        r#"
            INSERT INTO posts (author_id, space_id, title, content, media_url, media_type)
            VALUES ($1, $2, $3, $4, $5, $6)
            RETURNING *
            "#,
    )
    .bind(user.user_id)
    .bind(payload.space_id)
    .bind(&payload.title)
    .bind(&payload.content)
    .bind(&payload.media_url)
    .bind(&payload.media_type)
    .fetch_one(&state.db)
    .await?;

    Ok((StatusCode::CREATED, Json(post)))
}

pub async fn get_posts_by_space(
    State(state): State<AppState>,
    Path(space_id): Path<Uuid>,
) -> Result<(StatusCode, Json<Vec<Post>>), AppError> {
    let posts = sqlx::query_as::<_, Post>(
        "SELECT * FROM posts WHERE space_id = $1 ORDER BY created_at DESC",
    )
    .bind(space_id)
    .fetch_all(&state.db)
    .await?;

    Ok((StatusCode::OK, Json(posts)))
}
