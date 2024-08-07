mod task_gql_mutations;
mod user_gql_mutations;
mod login_gql_mutations;

use async_graphql::MergedObject;

#[derive(Default)]
pub struct UserMutations;

#[derive(Default)]
pub struct PostMutations;

#[derive(Default)]
pub struct TaskMutations;

#[derive(Default)]
pub struct LoginMutations;

#[derive(MergedObject, Default)]
pub struct Mutations(UserMutations, TaskMutations, LoginMutations);
