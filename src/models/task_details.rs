use async_graphql::SimpleObject;
use serde::{Deserialize, Serialize};

#[derive(SimpleObject, Serialize, Deserialize, Debug, Clone)]
pub struct TaskDetails{
    pub user_uuid: String,
    pub task_uuid: String,
    pub task_name: String,
    pub task_type: String,
    pub task_description: String,
    pub source_file: String
}