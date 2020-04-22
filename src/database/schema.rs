table! {
    api_users (user_id) {
        user_id -> Integer,
        username -> Varchar,
        password -> Nullable<Varchar>,
        auth_ip -> Nullable<Varchar>,
    }
}

table! {
    api_users_manuallycreated (user_id) {
        user_id -> Integer,
        username -> Varchar,
        password -> Nullable<Varchar>,
        auth_ip -> Nullable<Varchar>,
    }
}

table! {
    discord_users_blacklist (id) {
        id -> Integer,
        discord_userid -> Varchar,
        discord_id -> Bigint,
        added_by_id -> Bigint,
        guild_id -> Nullable<Bigint>,
        dt_created -> Nullable<Datetime>,
        dt_modified -> Nullable<Datetime>,
    }
}

table! {
    stats_seasons_history (id) {
        id -> Integer,
        user_id -> Integer,
        season -> Integer,
        season_rws -> Float,
        season_elo -> Float,
        season_rank -> Varchar,
        season_roundsplayed -> Integer,
    }
}

table! {
    stats_seasons_history_manualycreated (id) {
        id -> Integer,
        user_id -> Integer,
        season -> Integer,
        season_rws -> Float,
        season_elo -> Float,
        season_rank -> Varchar,
        season_roundsplayed -> Integer,
    }
}

table! {
    users (id) {
        id -> Integer,
        display_name -> Varchar,
        steamid2 -> Varchar,
        discord -> Nullable<Varchar>,
        elo -> Float,
        rws -> Float,
        rounds_total -> Integer,
    }
}

allow_tables_to_appear_in_same_query!(
    api_users,
    api_users_manuallycreated,
    discord_users_blacklist,
    stats_seasons_history,
    stats_seasons_history_manualycreated,
    users,
);
