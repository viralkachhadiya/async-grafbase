use async_graphql::InputObject;
use serde::{Deserialize, Serialize};

#[derive(InputObject, Serialize, Deserialize, Debug, Clone)]
pub struct TaskInput{
    pub user_uuid: String,
    pub task_uuid: String,
    pub task_name: String,
    pub task_type: String,
    pub task_description: String,
    pub source_file: String
}