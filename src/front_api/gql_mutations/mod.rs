use async_graphql::MergedObject;

#[derive(Default)]
pub struct UserMutations;
#[derive(Default)]
pub struct PostMutations;
#[derive(Default)]
pub struct LoginMutations;

#[derive(MergedObject, Default)]
pub struct Mutations;