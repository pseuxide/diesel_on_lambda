use diesel::prelude::*;
use diesel_on_lambda::models::User;
use diesel_on_lambda::*;

fn main() {
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
    println!("{}", serialized);
}
