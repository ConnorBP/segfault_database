use serde::{Serialize, Deserialize};
use super::schema::{users, api_users};

// Users DB Models

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

// API Authorization Users Models

#[derive(Queryable, Serialize, Deserialize)]
pub struct ApiUser {
    pub user_id: i32,
    pub username: String,
    pub password: Option<String>,
    pub auth_ip: Option<String>,
}

#[derive(Insertable)]
#[table_name="api_users"]
pub struct NewApiUser<'a> {
    pub username: &'a str,
    pub password: &'a str,
    pub auth_ip: &'a str,
}