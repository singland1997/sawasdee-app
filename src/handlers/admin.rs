use anyhow::Result;
use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use serde_json::{Value, json};
use uuid::Uuid;

use crate::{errors::AppError, middleware::auth::AuthUser, state::AppState};

pub async fn kick_user(
    State(state): State<AppState>,
    user: AuthUser,
    Path(target_user_id): Path<Uuid>,
) -> Result<(StatusCode, Json<Value>), AppError> {
    if user.role != "admin" {
        return Err(AppError::Unauthorized("Admin privileges required".into()));
    }

    if user.user_id == target_user_id {
        return Err(AppError::BadRequest("You cannot kick yourself".into()));
    }

    let result = sqlx::query("UPDATE users SET is_active = false WHERE id = $1")
        .bind(target_user_id)
        .execute(&state.db)
        .await?;

    if result.rows_affected() == 0 {
        return Err(AppError::NotFound("Target user not found".into()));
    }

    Ok((
        StatusCode::OK,
        Json(json!({
            "message": format!("User {} has been kicked out successfully", target_user_id)
        })),
    ))
}

pub async fn unban_user(
    State(state): State<AppState>,
    user: AuthUser,
    Path(target_user_id): Path<Uuid>,
) -> Result<(StatusCode, Json<Value>), AppError> {
    if user.role != "admin" {
        return Err(AppError::Unauthorized("Admin privileges required".into()));
    }

    let result = sqlx::query("UPDATE users SET is_active = true id = $1")
        .bind(target_user_id)
        .execute(&state.db)
        .await?;

    if result.rows_affected() == 0 {
        return Err(AppError::NotFound("Terget user not found".into()));
    }

    Ok((
        StatusCode::OK,
        Json(json!({
            "message": format!("User {} has been unbanned successfully", target_user_id)
        })),
    ))
}
