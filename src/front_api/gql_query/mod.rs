use async_graphql::MergedObject;

#[derive(Default)]
pub struct UserQuery;
#[derive(Default)]
pub struct LoginQuery;

#[derive(MergedObject, Default)]
pub struct Query;