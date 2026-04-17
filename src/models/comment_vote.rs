use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct CommentVote {
    pub id: Uuid,
    pub comment_id: Uuid,
    pub vote_type: i16,
}