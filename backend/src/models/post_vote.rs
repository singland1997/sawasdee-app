use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct PostVote {
    pub user_id: Uuid,
    pub post_id: Uuid,
    pub vote_type: i16,
}

#[derive(Debug, Deserialize)]
pub struct VoteReq {
    pub vote_type: i16,
}