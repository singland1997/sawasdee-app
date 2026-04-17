use anyhow::{anyhow, Result};
use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use crate::errors::AppError;
use crate::middleware::auth::AuthUser;
use crate::models::user::{AuthResponse, LoginUserReq, RegisterUserReq, User, UserResponse};
use crate::state::AppState;
use crate::utils::jwt::generate_token;
use crate::utils::password::{hash_password, verify_password};

pub async fn get_users(
    State(state): State<AppState>,
    user: AuthUser,
) -> Result<(StatusCode, Json<Vec<User>>), AppError> {
    tracing::info!("User {} is requesting the user list", user.user_id);

    let users = sqlx::query_as::<_, User>(
        "SELECT * FROM users"
    )
        .fetch_all(&state.db)
        .await?;

    Ok((StatusCode::OK, Json(users)))
}

pub async fn register_user(
    State(state): State<AppState>,
    Json(payload): Json<RegisterUserReq>,
) -> Result<(StatusCode, Json<UserResponse>), AppError> {
    let password_hash = hash_password(&payload.password).await?;

    let user = sqlx::query_as::<_, UserResponse>(
        r#"
        INSERT INTO users (username, email, password_hash)
        VALUES ($1, $2, $3)
        RETURNING id, username, email, created_at
        "#,
    )
        .bind(&payload.username)
        .bind(&payload.email)
        .bind(&password_hash)
        .fetch_one(&state.db)
        .await
        .map_err(|e| match e {
            sqlx::Error::Database(db_err) if db_err.is_unique_violation() => {
                AppError::Conflict("Username or email already exists".into())
            }
            e => AppError::from(e),
        })?;

    tracing::info!("New user registered with Argon2: {}", user.username);
    Ok((StatusCode::CREATED, Json(user)))
}

pub async fn login_user(
    State(state): State<AppState>,
    Json(payload): Json<LoginUserReq>,
) -> Result<(StatusCode, Json<AuthResponse>), AppError> {
    let user = sqlx::query_as::<_, User>(
        "SELECT * FROM users WHERE email = $1"
    )
        .bind(&payload.email)
        .fetch_one(&state.db)
        .await
        .map_err(|e| match e {
            sqlx::Error::RowNotFound => AppError::NotFound("Invalid email or password".to_string()),
            e => AppError::from(e),
        })?;

    verify_password(&payload.password, &user.password_hash).await?;

    let token = generate_token(&user.id, &state.jwt_secret)
        .map_err(|e| anyhow!("Failed to generate token: {}", e))?;

    let response = AuthResponse {
        token,
        user: UserResponse {
            id: user.id,
            username: user.username,
            email: user.email,
            created_at: user.created_at,
        },
    };

    Ok((StatusCode::OK, Json(response)))
}