use serde::{Serialize, Deserialize};
use super::schema::{users, api_users};
use chrono::NaiveDateTime;

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

// Discord users blacklist

pub struct DiscordBlacklistUser<'a> {
    pub id: i32,
    pub discord_userid: &'a str,
    pub discord_id: u64,
    pub added_by_id: u64,
    pub guild_id: Option<u64>,//null means its a global blacklist (only owner can add these)
    pub dt_created: Option<NaiveDateTime>,
    pub dt_modified: Option<NaiveDateTime>,
}

pub struct NewDiscordBlacklistUser<'a> {
    pub discord_userid: &'a str,
    pub discord_id: u64,
    pub added_by_id: u64,
    pub guild_id: Option<u64>,
}