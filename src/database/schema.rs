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
