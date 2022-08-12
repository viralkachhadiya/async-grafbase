pub mod task_query;

use async_graphql::MergedObject;

#[derive(Default, MergedObject)]
pub struct Query(
    task_query::TaskQuery
);