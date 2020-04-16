#[macro_use]
extern crate diesel;
extern crate dotenv;

// database models and schemas
pub mod models;
pub mod schema;

//deisel db
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use dotenv::dotenv;
use std::env;

// our models and schemas
use self::models::{NewUser, User, NewApiUser, ApiUser};

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

/// Generate a new apiuser in the database for authentication with the rest api
pub fn create_api_user<'a>(
    conn: &MysqlConnection,
    username: &'a str,
    password: &'a str,
    ip: &'a str,
) -> Result<Option<models::ApiUser>, diesel::result::Error> {
    use schema::api_users;
    use schema::api_users::dsl::user_id;

    let new_api_user = NewApiUser {
        username,
        password,
        auth_ip: ip,
    };

    diesel::insert_into(api_users::table)
        .values(&new_api_user)
        .execute(conn)
        .unwrap();

    let apiUser = api_users::dsl::api_users
        .order(user_id.desc())
        .first::<models::ApiUser>(conn)
        .optional()?;

    Ok(apiUser)
}

/// Generate a new user in the database
pub fn create_user<'a>(
    conn: &MysqlConnection,
    name: &'a str,
    steamid2: &'a str,
) -> Result<Option<models::User>, diesel::result::Error> {
    use schema::users;
    use schema::users::dsl::id;

    let new_user = NewUser {
        display_name: name,
        steamid2,
    };

    diesel::insert_into(users::table)
        .values(&new_user)
        .execute(conn)
        .unwrap();

    let user = users::dsl::users
        .order(id.desc())
        .first::<models::User>(conn)
        .optional()?;

    Ok(user)
}

// Create a new user only if it doesn't already exist for given steam id and return the found/created users stats
pub fn init_user<'a>(
    conn: &MysqlConnection,
    name: &'a str,
    _steamid2: &'a str,
) -> Result<models::User, diesel::result::Error> {
    use schema::users;
    use schema::users::dsl::{id, steamid2};

    if let Some(user) = users::dsl::users
        .filter(steamid2.eq(_steamid2))
        .first::<models::User>(conn)
        .optional()?
    {
        println!("User existed, returning.");
        Ok(user)
    } else {
        println!("User didn't exist, creating a new one.");
        let new_user = NewUser {
            display_name: name,
            steamid2: _steamid2,
        };

        diesel::insert_into(users::table)
            .values(&new_user)
            .execute(conn)
            .unwrap();

        let user = users::dsl::users
            .order(id.desc())
            .first::<models::User>(conn);
        user
    }
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
    use self::models::*;
    use self::schema::users::dsl::{rounds_total, rws, users};
    use diesel::prelude::*;
    diesel::update(users.find(_id))
        .set((rws.eq(newRws), rounds_total.eq(rounds_total + 1)))
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
