use crate::{models::task_details::TaskDetails, services, utils::gql_errors::GqlError};
use async_graphql::{Context, Error, ErrorExtensions, Object, Result};
use aws_sdk_dynamodb::Client;
use tracing::error;

#[derive(Default)]
pub struct TaskQuery;

#[Object]
impl TaskQuery {
    ///Get all task details
    pub async fn get_tasks(&self, ctx: &Context<'_>) -> Result<Vec<TaskDetails>, Error> {
        let client = match ctx.data::<Client>() {
            Ok(c) => c,
            Err(_) => {
                error!("Couldn't establish database connection.");
                return Err(
                    GqlError::InternalServerError("Internal server error".to_string()).extend(),
                );
            }
        };
        return match services::task::get_tasks(client).await {
            Ok(t) => Ok(t),
            Err(_) => {
                Err(GqlError::InternalServerError("Internal server error".to_string()).extend())
            }
        };
    }

    ///Get task details based on id
    pub async fn get_task_by_id(
        &self,
        ctx: &Context<'_>,
        task_id: String,
    ) -> Result<TaskDetails, Error> {
        let client = match ctx.data::<Client>() {
            Ok(c) => c,
            Err(_) => {
                error!("Couldn't establish database connection.");
                return Err(
                    GqlError::InternalServerError("Internal server error".to_string()).extend(),
                );
            }
        };
        return match services::task::get_task_by_id(client, task_id).await {
            Ok(t) => {
                return match t {
                    Some(r) => Ok(r),
                    None => Err(GqlError::NotFound("Not found".to_string()).extend()),
                }
            }
            Err(_) => {
                Err(GqlError::InternalServerError("Internal server error".to_string()).extend())
            }
        };
    }
}
