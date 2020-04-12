use serde::{Serialize, Deserialize};
use super::schema::users;

#[derive(Queryable, Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub display_name: String,
    pub steamid2: String,
    pub discord: Option<String>,
    pub elo: f32,
    pub rws: f32,
    pub rounds_total: i32,
}

#[derive(Insertable)]
#[table_name="users"]
pub struct NewUser<'a> {
    pub display_name: &'a str,
    pub steamid2: &'a str,
}