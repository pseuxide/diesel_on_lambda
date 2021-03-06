use diesel::prelude::*;
use diesel_on_lambda::models::User;
use diesel_on_lambda::*;
use lambda_runtime::{handler_fn, Context, Error};
use serde_json::Value;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let func = handler_fn(func);
    lambda_runtime::run(func).await?;
    Ok(())
}

pub(crate) async fn func(_event: Value, _: Context) -> Result<String, Error> {
    use diesel_on_lambda::schema::user::dsl::*;

    let connection = &mut establish_connection();

    let results = user
        .filter(address.eq("Documental"))
        .limit(5)
        .load::<User>(connection)
        .expect("Error loading posts");

    //for usere in results {
    //    println!("{} | {} | {}", usere.id, usere.name, usere.address);
    //}
    let serialized = serde_json::to_string(&results).expect("Failed to serialize data");
    Ok(serialized)
}
