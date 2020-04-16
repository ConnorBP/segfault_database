extern crate diesel;
extern crate sfdb_connect;

use sfdb_connect::models::{ApiUser, NewApiUser};
use diesel::prelude::*;
use sfdb_connect::*;
use std::io::{stdin, Read};
use rpassword::read_password;

fn main() {
    let connection = establish_connection();

    let mut username = String::new();
    let mut ip = String::new();

    println!("What would you like the username to be?");
    stdin().read_line(&mut username).unwrap();
    let username = username.trim_end(); // Remove the trailing newline

    println!("What is the password for this new authorized user? (optional)");
    let passwd = read_password().unwrap_or("".to_string());// get passwd input in a secure way  

    println!("What ip would you like this auth to be locked to? (optional)");
    stdin().read_line(&mut ip).unwrap();
    let ip = ip.trim_end();

    let apiUser = create_api_user(&connection, &username, &passwd, &ip).map_err(|e| {
        eprintln!("{}", e);
    }).unwrap();
    
    if let Some(apiUser) = apiUser {
        println!("\n--------\nSaved user {} with id {}", username, apiUser.user_id);
    } else {
        println!("Error Creating new user!");
    }
}