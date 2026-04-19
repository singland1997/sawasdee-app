use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Post {
    pub id: Uuid,
    pub author_id: Uuid,
    pub space_id: Uuid,
    pub title: String,
    pub content: Option<String>,
    pub media_url: Option<String>,
    pub media_type: Option<String>,
    pub upvote_count: i32,
    pub comment_count: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct CreatePostReq {
    pub space_id: Uuid,
    pub title: String,
    pub content: Option<String>,
    pub media_url: Option<String>,
    pub media_type: Option<String>,
}