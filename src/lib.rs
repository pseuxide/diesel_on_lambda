#[macro_use]
extern crate diesel;
extern crate dotenv;

pub mod models;
pub mod schema;

use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

pub fn establish_connection() -> MysqlConnection {
    dotenv().ok();

    //env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let database_url = "mysql://s3pt3mb3r:Am4ne5262521@test-db.cef4eexsaboa.ap-northeast-1.rds.amazonaws.com:3306/user_info".to_string();
    MysqlConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}