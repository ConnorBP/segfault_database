extern crate diesel;
extern crate sfdb_connect;

use sfdb_connect::models::*;
use diesel::prelude::*;
use sfdb_connect::*;
use std::io::{stdin, Read};

fn main() {
    let connection = establish_connection();

    let mut name = String::new();
    let mut steam = String::new();

    println!("What would you like the username to be?");
    stdin().read_line(&mut name).unwrap();
    let name = name.trim_end(); // Remove the trailing newline

    println!("What is the steam ID (STEAM_0:1:) of this user?");
    stdin().read_line(&mut steam).unwrap();
    let steam = steam.trim_end(); // Remove the trailing newline

    /*println!(
        "\nOk! Let's write {} (Press {} when finished)\n",
        title, EOF
    );
    stdin().read_to_string(&mut body).unwrap();*/

    let user = create_user(&connection, name, &steam).map_err(|e| {
        eprintln!("{}", e);
        HttpResponse::InternalServerError().finish()
    })?;
    println!("\nSaved user {} with id {}", name, user.id);
}

//#[cfg(not(windows))]
//const EOF: &str = "CTRL+D";

//#[cfg(windows)]
//const EOF: &str = "CTRL+Z";