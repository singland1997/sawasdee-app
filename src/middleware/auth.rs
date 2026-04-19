use crate::errors::AppError;
use crate::state::AppState;
use crate::utils::jwt::verify_token;
use axum::extract::FromRequestParts;
use axum::http::header;
use axum::http::request::Parts;
use uuid::Uuid;

#[derive(Debug)]
pub struct AuthUser {
    pub user_id: Uuid,
    pub role: String,
}

impl FromRequestParts<AppState> for AuthUser {
    type Rejection = AppError;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        let auth_header = parts
            .headers
            .get(header::AUTHORIZATION)
            .and_then(|value| value.to_str().ok())
            .ok_or_else(|| AppError::Unauthorized("Missing Authorization header".into()))?;

        if !auth_header.starts_with("Bearer ") {
            return Err(AppError::Unauthorized(
                "Invalid authorization header format".into(),
            ));
        }

        let token = &auth_header.trim_start_matches("Bearer ");

        let claims = verify_token(token, &state.jwt_secret)
            .map_err(|_| AppError::Unauthorized("Invalid or expired token".into()))?;

        let user_id = Uuid::parse_str(&claims.sub)
            .map_err(|_| AppError::Unauthorized("Invalid user ID in token".into()))?;

        let user_record: Option<(String, bool)> = sqlx::query_as(
            "SELECT role, is_active FROM users WHERE id = $1"
        )
            .bind(user_id)
            .fetch_optional(&state.db)
            .await
            .map_err(|e| {
                tracing::error!("Auth DB Error: {}", e);
                AppError::Unauthorized("Database error during authentication".into())
            })?;

        match user_record {
            Some((_, is_active)) if !is_active => {
                Err(AppError::Unauthorized("Your account has been deactivated".into()))
            }
            Some((role, _)) => Ok(AuthUser { user_id, role }),
            None => Err(AppError::Unauthorized("User no longer exists".into())),
        }
    }
}
