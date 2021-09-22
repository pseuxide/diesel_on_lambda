use diesel::Queryable;
use serde::Serialize;

#[derive(Queryable, Serialize, Debug)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub address: String,
}
