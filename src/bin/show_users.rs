extern crate diesel;
extern crate sfdb_connect;

use sfdb_connect::models::*;
use diesel::prelude::*;
use sfdb_connect::*;


fn main() {
    use sfdb_connect::schema::users::dsl::*;

    let connection = establish_connection();
    let results = users
        .filter(rounds_total.ge(0))
        .limit(5)
        .load::<User>(&connection)
        .expect("Error loading users:");

    println!("Displaying {} users\n", results.len());
    for user in results {
        println!("{}: {}", user.id, user.display_name);
        println!("-----------\n");
        println!("{}", user.steamid2);
        println!("{}rws with {} rounds played.\n", user.rws, user.rounds_total);
        println!("-----------\n");
    }
}