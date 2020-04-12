extern crate diesel;
extern crate sfdb_connect;

use sfdb_connect::models::*;
use diesel::prelude::*;
use sfdb_connect::*;
use std::io::{stdin, Read};

use std::env::args;

fn main() {

    // WARNING! DANGEROUS! WILL DELETE ALL USERS WITH SIMILAR NAME.

    // NEVER EVER USE THIS ON A LIVE DATABASE!


    //use self::schema::users::dsl::*;//either should work
    use sfdb_connect::schema::users::dsl::*;

    let target = args().nth(1).expect("Expected a target to match against");
    let pattern = format!("%{}%", target);

    let connection = establish_connection();
    let num_deleted = diesel::delete(users.filter(display_name.like(pattern)))
        .execute(&connection)
        .expect("Error deleting users");

    println!("Deleted {} users", num_deleted);
}