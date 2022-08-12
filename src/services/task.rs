use std::collections::HashMap;

use aws_sdk_dynamodb::model::AttributeValue;
use aws_sdk_dynamodb::Client;

use anyhow::anyhow;
use tracing::error;

use crate::models::task_details::TaskDetails;
use crate::models::task_input::TaskInput;
use crate::models::task_response::TaskResponse;

///Create task
pub async fn create_task_async(
    client: &Client,
    task: TaskInput,
) -> Result<TaskResponse, anyhow::Error> {
    let request = client
        .put_item()
        .table_name("task")
        .item("pK", AttributeValue::S(String::from(task.user_uuid)))
        .item("sK", AttributeValue::S(String::from(task.task_uuid)))
        .item("name", AttributeValue::S(String::from(task.task_name)))
        .item(
            "description",
            AttributeValue::S(String::from(task.task_description)),
        )
        .item("type", AttributeValue::S(String::from(task.task_type)))
        .item(
            "source_file",
            AttributeValue::S(String::from(task.source_file)),
        );

    return match request.send().await {
        Ok(_) => Ok(TaskResponse {
            status: String::from("success"),
            message: String::from("Task ccreated successfully.."),
        }),
        Err(e) => Err(anyhow!(e)),
    };
}

///Getting all tasks
pub async fn get_tasks(client: &Client) -> Result<Vec<TaskDetails>, anyhow::Error> {
    let res = client.query().table_name("task").send().await;
    return match res {
        Ok(output) => match output.items {
            Some(items) => {
                let mut task_items = Vec::new();
                for item in items.iter() {
                    error!("{:?}", &item);
                    let task = match item_to_task(item) {
                        Ok(task) => task,
                        Err(e) => {
                            return Err(anyhow!(e));
                        }
                    };
                    task_items.push(task);
                }
                Ok(task_items)
            }
            None => Ok(vec![]),
        },
        Err(error) => {
            error!("{:?}", error);
            Err(anyhow!(error))
        }
    };
}

///Get task by id
pub async fn get_task_by_id(
    client: &Client,
    task_id: String,
) -> Result<Option<TaskDetails>, anyhow::Error> {
    let tokens: Vec<String> = task_id.split("_").map(|x| String::from(x)).collect();
    let user_uuid = AttributeValue::S(tokens[0].clone());
    let task_uuid = AttributeValue::S(tokens[1].clone());

    let res = client
        .query()
        .table_name("task")
        .key_condition_expression("#pK = :user_id and #sK = :task_uuid")
        .expression_attribute_names("#pK", "pK")
        .expression_attribute_names("#sK", "sK")
        .expression_attribute_values(":user_id", user_uuid)
        .expression_attribute_values(":task_uuid", task_uuid)
        .send()
        .await;

    return match res {
        Ok(output) => match output.items {
            Some(items) => {
                let item = &items.first().unwrap();
                error!("{:?}", &item);
                return match item_to_task(item) {
                    Ok(task) => Ok(Some(task)),
                    Err(e) => Err(anyhow!(e)),
                };
            }
            None => Ok(None),
        },
        Err(error) => {
            error!("{:?}", error);
            Err(anyhow!(error))
        }
    };
}

///Delete task by id
pub async fn delete_task_by_id(client: &Client, task_id: String) -> Result<bool, anyhow::Error> {
    let tokens: Vec<String> = task_id.split("_").map(|x| String::from(x)).collect();
    let user_uuid = AttributeValue::S(tokens[0].clone());
    let task_uuid = AttributeValue::S(tokens[1].clone());

    return match client
        .delete_item()
        .table_name("task")
        .key("pK", user_uuid.into())
        .key("sK", task_uuid.into())
        .send()
        .await
    {
        Ok(_) => Ok(true),
        Err(e) => {
            error!("{:?}", e);
            Err(anyhow!(e))
        }
    };
}

///Converty item into task details object
fn item_to_task(item: &HashMap<String, AttributeValue>) -> Result<TaskDetails, anyhow::Error> {
    Ok(TaskDetails {
        user_uuid: required_item_value("pK", item)?,
        task_uuid: required_item_value("sK", item)?,
        task_name: required_item_value("name", item)?,
        task_description: required_item_value("description", item)?,
        task_type: required_item_value("type", item)?,
        source_file: required_item_value("source_file", item)?,
    })
}

fn required_item_value(
    key: &str,
    item: &HashMap<String, AttributeValue>,
) -> Result<String, anyhow::Error> {
    match item_value(key, item) {
        Ok(Some(value)) => Ok(value),
        Ok(None) => Err(anyhow!("Something went wrong..".to_string())),
        Err(e) => Err(anyhow!(e)),
    }
}

fn item_value(
    key: &str,
    item: &HashMap<String, AttributeValue>,
) -> Result<Option<String>, anyhow::Error> {
    match item.get(key) {
        Some(value) => match value.as_s() {
            Ok(val) => Ok(Some(val.clone())),
            Err(_) => Err(anyhow!("Something went wrong..".to_string())),
        },
        None => Ok(None),
    }
}
