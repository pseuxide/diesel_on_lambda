use diesel::prelude::*;
use diesel_test::models::User;
use diesel_test::*;

fn main() {
    use diesel_test::schema::user::dsl::*;

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
