use crate::errors::AppError;
use crate::middleware::auth::AuthUser;
use crate::models::comment::{Comment, CreateCommentReq};
use crate::state::AppState;
use anyhow::Result;
use axum::Json;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use uuid::Uuid;

pub async fn create_comment(
    State(state): State<AppState>,
    user: AuthUser,
    Path(post_id): Path<Uuid>,
    Json(payload): Json<CreateCommentReq>,
) -> Result<(StatusCode, Json<Comment>), AppError> {
    let mut tx = state.db.begin().await?;

    let comment = sqlx::query_as::<_, Comment>(
        r#"
            INSERT INTO comments (post_id, author_id, parent_id, content)
            VALUES ($1, $2, $3, $4)
            RETURNING *
            "#,
    )
    .bind(post_id)
    .bind(user.user_id)
    .bind(payload.parent_id)
    .bind(&payload.content)
    .fetch_one(&mut *tx)
    .await?;

    sqlx::query("UPDATE  posts SET comment_count = comment_count + 1 WHERE id = $1")
        .bind(post_id)
        .execute(&mut *tx)
        .await?;

    tx.commit().await?;

    Ok((StatusCode::CREATED, Json(comment)))
}

pub async fn get_post_comments(
    State(state): State<AppState>,
    Path(post_id): Path<Uuid>,
) -> Result<(StatusCode, Json<Vec<Comment>>), AppError> {
    let comments = sqlx::query_as::<_, Comment>(
        "SELECT * FROM comments WHERE post_id = $1 ORDER BY created_at ASC",
    )
    .bind(post_id)
    .fetch_all(&state.db)
    .await?;

    Ok((StatusCode::OK, Json(comments)))
}
