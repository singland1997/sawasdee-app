use axum::{
    extract::Multipart,
    http::StatusCode,
    Json,
};
use serde_json::{json, Value};
use tokio::fs::File;
use tokio::io::AsyncWriteExt;
use std::path::Path;
use uuid::Uuid;

use crate::{
    errors::AppError,
    middleware::auth::AuthUser,
};

pub async fn upload_media(
    _user: AuthUser,
    mut multipart: Multipart,
) -> Result<(StatusCode, Json<Value>), AppError> {

    while let Some(field) = multipart
        .next_field()
        .await
        .map_err(|_| AppError::BadRequest("Invalid multipart data".into()))?
    {
        let content_type = field.content_type().map(ToString::to_string);
        let file_name = field.file_name().map(ToString::to_string);

        if let Some(name) = file_name {
            let ext = Path::new(&name)
                .extension()
                .and_then(|e| e.to_str())
                .unwrap_or("bin");

            let unique_name = format!("{}.{}", Uuid::new_v4(), ext);
            let save_path = format!("./uploads/{}", unique_name);

            let data = field
                .bytes()
                .await
                .map_err(|_| AppError::BadRequest("Failed to read file stream".into()))?;

            let mut file = File::create(&save_path)
                .await
                .map_err(|e| {
                    tracing::error!("Cannot create file: {}", e);
                    AppError::Anyhow(anyhow::anyhow!("Internal Server Error: Cannot save file"))
                })?;

            file.write_all(&data)
                .await
                .map_err(|e| {
                    tracing::error!("Cannot write file: {}", e);
                    AppError::Anyhow(anyhow::anyhow!("Internal Server Error: Cannot write file"))
                })?;

            let media_url = format!("/uploads/{}", unique_name);

            return Ok((
                StatusCode::CREATED,
                Json(json!({
                    "message": "File uploaded successfully",
                    "media_url": media_url,
                    "media_type": content_type
                })),
            ));
        }
    }

    Err(AppError::BadRequest("No file found in the request".into()))
}