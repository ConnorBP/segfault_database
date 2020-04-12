extern crate diesel;
extern crate sfdb_connect;

use sfdb_connect::models::*;
use diesel::prelude::*;
use sfdb_connect::*;

use std::env::args;


fn main() {
    use self::schema::users::dsl::{users, rounds_total};

    let id = args()
        .nth(1)
        .expect("update_user requires a user id")
        .parse::<i32>()
        .expect("Invalid ID");
    let connection = establish_connection();

    let user: User = users
        .find(id)
        .first(&connection)
        .unwrap_or_else(|_| panic!("Unable to find user {}", id));

    diesel::update(users.find(id))
        .set(rounds_total.eq(rounds_total + 1))
        .execute(&connection)
        .unwrap();

    println!("Updated user {}", user.display_name);
}