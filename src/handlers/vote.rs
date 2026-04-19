use crate::errors::AppError;
use crate::middleware::auth::AuthUser;
use crate::models::post_vote::VoteReq;
use crate::state::AppState;
use anyhow::Result;
use axum::Json;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use serde_json::{Value, json};
use uuid::Uuid;

pub async fn vote_post(
    State(state): State<AppState>,
    user: AuthUser,
    Path(post_id): Path<Uuid>,
    Json(payload): Json<VoteReq>,
) -> Result<(StatusCode, Json<Value>), AppError> {
    let mut tx = state.db.begin().await?;

    let old_vote: Option<i16> =
        sqlx::query_scalar("SELECT vote_type FROM post_votes WHERE user_id = $1 AND post_id = $2")
            .bind(user.user_id)
            .bind(post_id)
            .fetch_optional(&mut *tx)
            .await?;

    let old_vote_val = old_vote.unwrap_or(0);
    let new_vote_val = payload.vote_type;

    let score_diff = (new_vote_val as i32) - (old_vote_val as i32);

    if score_diff != 0 {
        sqlx::query(
            r#"
            INSERT INTO post_votes (user_id, post_id, vote_type)
            VALUES ($1, $2, $3)
            ON CONFLICT (user_id, post_id) DO UPDATE
            SET vote_type = EXCLUDED.vote_type
            "#,
        )
        .bind(user.user_id)
        .bind(post_id)
        .bind(new_vote_val)
        .execute(&mut *tx)
        .await?;

        sqlx::query("UPDATE posts SET upvote_count = upvote_count + $1 WHERE id = $2")
            .bind(score_diff)
            .bind(post_id)
            .execute(&mut *tx)
            .await?;
    }

    tx.commit().await?;

    Ok((
        StatusCode::OK,
        Json(json!({
            "message": "Voted successfully",
            "score_diff": score_diff
        })),
    ))
}

pub async fn vote_comment(
    State(state): State<AppState>,
    user: AuthUser,
    Path(comment_id): Path<Uuid>,
    Json(payload): Json<VoteReq>,
) -> Result<(StatusCode, Json<Value>), AppError> {
    let mut tx = state.db.begin().await?;

    let old_vote: Option<i16> =
        sqlx::query_scalar("SELECT vote_type FROM comment_votes WHERE user_id = $1 AND comment_id = $2")
            .bind(user.user_id)
            .bind(comment_id)
            .fetch_optional(&mut *tx)
            .await?;

    let old_vote_val = old_vote.unwrap_or(0);
    let new_vote_val = payload.vote_type;

    let score_diff = (new_vote_val as i32) - (old_vote_val as i32);

    if score_diff != 0 {
        sqlx::query(
            r#"
            INSERT INTO comment_votes (user_id, comment_id, vote_type)
            VALUES ($1, $2, $3)
            ON CONFLICT (user_id, comment_id) DO UPDATE
            SET vote_type = EXCLUDED.vote_type
            "#,
        )
            .bind(user.user_id)
            .bind(comment_id)
            .bind(new_vote_val)
            .execute(&mut *tx)
            .await?;

        sqlx::query("UPDATE comments SET upvote_count = upvote_count + $1 WHERE id = $2")
            .bind(score_diff)
            .bind(comment_id)
            .execute(&mut *tx)
            .await?;
    }

    tx.commit().await?;

    Ok((
        StatusCode::OK,
        Json(json!({
            "message": "Voted successfully",
            "score_diff": score_diff
        })),
    ))
}
