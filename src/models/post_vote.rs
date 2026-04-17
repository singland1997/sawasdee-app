use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct PostVote {
    pub id: Uuid,
    pub user_id: Uuid,
    pub post_id: Uuid,
    pub vote_type: i16,
}