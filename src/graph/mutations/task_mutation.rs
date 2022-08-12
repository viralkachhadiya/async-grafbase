use async_graphql::{Context, Error, ErrorExtensions, Object, Result};
use aws_sdk_dynamodb::Client;
use tracing::error;

use crate::{
    models::{task_input::TaskInput, task_response::TaskResponse},
    services,
    utils::gql_errors::GqlError,
};

#[derive(Default)]
pub struct TaskMutation;

#[Object]
impl TaskMutation {
    ///Create task
    pub async fn create_task(
        &self,
        ctx: &Context<'_>,
        task: TaskInput,
    ) -> Result<TaskResponse, Error> {
        let client = match ctx.data::<Client>() {
            Ok(c) => c,
            Err(_) => {
                error!("Couldn't establish database connection.");
                return Err(
                    GqlError::InternalServerError("Internal server error".to_string()).extend(),
                );
            }
        };
        return match services::task::create_task_async(client, task).await {
            Ok(r) => Ok(r),
            Err(_) => {
                Err(GqlError::InternalServerError("Internal server error".to_string()).extend())
            }
        };
    }

    ///Delete task
    pub async fn delete_task(&self, ctx: &Context<'_>, task_id: String) -> Result<bool, Error> {
        let client = match ctx.data::<Client>() {
            Ok(c) => c,
            Err(_) => {
                error!("Couldn't establish database connection.");
                return Err(
                    GqlError::InternalServerError("Internal server error".to_string()).extend(),
                );
            }
        };
        return match services::task::delete_task_by_id(client, task_id).await {
            Ok(t) => Ok(t),
            Err(_) => {
                Err(GqlError::InternalServerError("Internal server error".to_string()).extend())
            }
        };
    }

    ///Update task details
    pub async fn update_task(&self, ctx: &Context<'_>, _id: String) -> Result<String, Error> {
        let _client = match ctx.data::<Client>() {
            Ok(c) => c,
            Err(_) => {
                error!("Couldn't establish database connection.");
                return Err(
                    GqlError::InternalServerError("Internal server error".to_string()).extend(),
                );
            }
        };
        Ok("soon".to_string())
    }
}
