#[macro_use]
extern crate diesel;
extern crate dotenv;

// database models and schemas
pub mod schema;
pub mod models;

//deisel db
use diesel::prelude::*;
use dotenv::dotenv;
use std::env;
use diesel::r2d2::{self, ConnectionManager};

// our models and schemas
use self::models::{User, NewUser};

// The type for our connection pool
pub type DbPool = r2d2::Pool<ConnectionManager<MysqlConnection>>;

/// Initialize the connection to our database
pub fn establish_connection() -> MysqlConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    MysqlConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub fn establish_connection_pool() -> DbPool {
    let connspec = std::env::var("DATABASE_URL").expect("DATABASE_URL");
    let manager = ConnectionManager::<MysqlConnection>::new(&connspec);
    r2d2::Pool::builder()
        .build(manager)
        .unwrap_or_else(|_| panic!("Error creating pool with {}", connspec))
}

/// Generate a new user in the database
pub fn create_user<'a>(conn: &MysqlConnection, name: &'a str, steamid2: &'a str) -> User {
    use schema::users;
    use schema::users::dsl::id;


    let new_user = NewUser {
        display_name: name,
        steamid2,
    };

    diesel::insert_into(users::table)
        .values(&new_user)
        .execute(conn)
        .expect("Error saving new user");

    users::dsl::users.order(id.desc()).first(conn).unwrap()
}

/// Run query using Diesel to insert a new database row and return the result.
pub fn find_user_by_id(
    conn: &MysqlConnection,
    _id: i32,
) -> Result<Option<models::User>, diesel::result::Error> {
    use crate::schema::users::dsl::*;

    let user = users
        .filter(id.eq(_id))
        .first::<models::User>(conn)
        .optional()?;

    Ok(user)
}

pub fn find_user_by_steam(
    conn: &MysqlConnection,
    _steamid: String,
) -> Result<Option<models::User>, diesel::result::Error> {
    use crate::schema::users::dsl::*;

    let user = users
        .filter(steamid2.eq(_steamid))
        .first::<models::User>(conn)
        .optional()?;

    Ok(user)
}

/// Increments the total_rounds, updates the rws value, then fetches and returns the latest values in the database
pub fn update_newround_user_by_id(
    conn: &MysqlConnection,
    _id: i32,
    newRws: f32,
) -> Result<Option<models::User>, diesel::result::Error> {
    use self::schema::users::dsl::{users, rounds_total, rws};
    use self::models::*;
    use diesel::prelude::*;
    diesel::update(users.find(_id))
        .set(rounds_total.eq(rounds_total + 1))
        //.set(rws.eq(newRws))
        .execute(conn)
        .unwrap();
    
    find_user_by_id(conn, _id)
}


// Actix Syncronous Actor Stuff
// ----------------------------

/*use actix::prelude::*;
use std::io::Error;

struct DbExecutor(MysqlConnection);

impl Actor for DbExecutor {
    type Context = SyncContext<Self>;
}

impl Message for NewUser<'_> {
    type Result = Result<User, Error>;
}

impl Handler<NewUser<'_>> for DbExecutor {
    type Result = Result<User, Error>;

    fn handle<'a>(&mut self, msg: NewUser, _: &mut Self::Context) -> Self::Result {
        Ok(create_user(&self.0, &msg.display_name, &msg.steamid2))
    }
}*/