use crate::errors::AppError;
use crate::middleware::auth::AuthUser;
use crate::models::space::{CreateSpaceReq, Space, SpaceMember};
use crate::state::AppState;
use anyhow::Result;
use axum::Json;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use uuid::Uuid;

pub async fn create_space(
    State(state): State<AppState>,
    user: AuthUser,
    Json(payload): Json<CreateSpaceReq>,
) -> Result<(StatusCode, Json<Space>), AppError> {
    let mut tx = state.db.begin().await?;

    let space = sqlx::query_as::<_, Space>(
        r#"
            INSERT INTO spaces (name, description, owner_id)
            Values ($1, $2, $3)
            RETURNING *
            "#,
    )
    .bind(&payload.name)
    .bind(&payload.description)
    .bind(user.user_id)
    .fetch_one(&mut *tx)
    .await
    .map_err(|e| match e {
        sqlx::Error::Database(db_err) if db_err.is_unique_violation() => {
            AppError::Conflict("Space name already exists".into())
        }
        e => AppError::from(e),
    })?;

    sqlx::query(
        r#"
            INSERT INTO space_members (space_id, user_id, role)
            VALUES ($1, $2, $3)
            "#,
    )
    .bind(space.id)
    .bind(user.user_id)
    .execute(&mut *tx)
    .await?;

    tx.commit().await?;

    Ok((StatusCode::CREATED, Json(space)))
}

pub async fn get_space(
    State(state): State<AppState>,
) -> Result<(StatusCode, Json<Vec<Space>>), AppError> {
    let spaces = sqlx::query_as::<_, Space>("SELECT * FROM spaces ORDER BY created_at DESC")
        .fetch_all(&state.db)
        .await?;

    Ok((StatusCode::OK, Json(spaces)))
}

pub async fn join_space(
    State(state): State<AppState>,
    user: AuthUser,
    Path(space_id): Path<Uuid>,
) -> Result<(StatusCode, Json<SpaceMember>), AppError> {
    let member = sqlx::query_as::<_, SpaceMember>(
        r#"
        INSERT INTO space_members (space_id, user_id, role)
        VALUES ($1, $2, $3)
        RETURNING *
        "#,
    )
    .bind(space_id)
    .bind(user.user_id)
    .fetch_one(&state.db)
    .await
    .map_err(|e| match e {
        sqlx::Error::Database(db_err) if db_err.is_unique_violation() => {
            AppError::Conflict("You are already a member of this space".into())
        }
        e => AppError::from(e),
    })?;

    Ok((StatusCode::CREATED, Json(member)))
}
