use async_graphql::{ComplexObject, SimpleObject};
use chrono;
use serde::{Deserialize, Serialize};

///#[derive(SimpleObject, Deserialize, Serialize)]
///#[graphql(complex)]

pub struct User {
    pub i: i32,
}
