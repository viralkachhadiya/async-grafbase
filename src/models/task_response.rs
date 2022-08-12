use async_graphql::SimpleObject;
use serde::{Deserialize, Serialize};

#[derive(SimpleObject, Serialize, Deserialize, Debug, Clone)]
pub struct TaskResponse{
    pub status: String,
    pub message: String,
}