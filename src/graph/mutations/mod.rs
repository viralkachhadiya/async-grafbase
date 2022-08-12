pub mod task_mutation;

use async_graphql::MergedObject;

#[derive(Default, MergedObject)]
pub struct MutationRoot(task_mutation::TaskMutation);
