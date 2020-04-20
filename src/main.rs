#[macro_use]
extern crate diesel;
extern crate dotenv;

mod rws;

//actix-web
use actix_web::{
    get, http, middleware, post, web, App, Error, HttpRequest, HttpResponse, HttpServer,
};
// http authentication
use actix_web::dev::ServiceRequest;
use actix_web_httpauth::extractors::basic::BasicAuth;
use actix_web_httpauth::extractors::{basic, bearer, AuthenticationError};
use actix_web_httpauth::middleware::HttpAuthentication;

use serde::{Deserialize, Serialize};

use diesel::prelude::*;
use sfdb_connect::models::*;
use sfdb_connect::*;

use diesel::r2d2::{self, ConnectionManager};

// Request Authorizor
async fn validator(req: ServiceRequest, _credentials: BasicAuth) -> Result<ServiceRequest, Error> {
    Ok(req)
    // if (_credentials.user_id() == "dank" && _credentials.password().unwrap().to_owned() == "memes") {
    //     //let resp = req.build_response(http::StatusCode::OK).body("AUTHORIZED");
    //     Ok(req)
    // } else {
    //     Err(AuthenticationError::from(basic::Config::default()).into())
    // }
}

// check if the api server is live (doesn't check the database)
async fn check(req: HttpRequest) -> &'static str {
    println!("Got check REQ: {:?}", req);
    "live"
}

#[derive(Deserialize)]
pub struct AuthRequest {
    name: String,
}

async fn stream_auth(req: HttpRequest, web::Query(info): web::Query<AuthRequest>) -> HttpResponse {
    println!("received stream name param: {:?}", info.name);
    HttpResponse::TemporaryRedirect()
        .header("Location", "fetched-username-from-key-here")
        .body("AUTHENTICATION SUCCESS")
}

// ranks api: /ranks (set,get,put,post)
// /stats/{userid}
// part of the ranks api. Returns stats for given steam id
#[get("/stats/{id}")]
async fn stats_get_by_id(
    pool: web::Data<DbPool>,
    id: web::Path<i32>,
) -> Result<HttpResponse, Error> {
    let id = id.into_inner();
    let conn = pool.get().expect("couldn't get db connection from pool");
    // use web::block to offload blocking Diesel code without blocking server thread
    let user = web::block(move || sfdb_connect::find_user_by_id(&conn, id))
        .await
        .map_err(|e| {
            eprintln!("{}", e);
            HttpResponse::InternalServerError().finish()
        })?;

    if let Some(user) = user {
        Ok(HttpResponse::Ok().json(user))
    } else {
        let res = HttpResponse::NotFound().body(format!("No user found with id: {}", id));
        Ok(res)
    }
}

// sends data, expects it to be handled
async fn post_stats() -> &'static str {
    "yote"
}

// put will create if not exists or replace if does exist
async fn put_stats() -> &'static str {
    "heyooo"
}


#[derive(Deserialize)]
pub struct UserInit {
    display_name: String,
    steam_id: String,
}
#[post("/userinit")]
async fn post_user_init(
    pool: web::Data<DbPool>,
    web::Form(userData): web::Form<UserInit>
) -> Result<HttpResponse, Error> {
    let conn = pool.get().expect("couldn't get db connection from pool");

    let steam_id_clone = userData.steam_id.clone();
    let user = web::block(move || sfdb_connect::init_user(&conn, &userData.display_name, &userData.steam_id))
                .await
                .map_err(|e| {
                    eprintln!("{}", e);
                    HttpResponse::InternalServerError().finish()
                })?;

    Ok(HttpResponse::Ok().json(user))
}



// receives stats from latest round (such as round points) with steam id
// points are then calculated into rws and merged with existing stats
// if successful, returns success and the newest calculated RWS value and rounds total
// Required Inputs: auth, SteamID, round points (round count is auto incremented)

#[derive(Deserialize)]
pub struct RoundData {
    steam_id: String,
    did_win: bool,
    round_points: i32,
    team_points: i32,
    team_count: i32,
}

#[post("/newround")]
async fn post_new_round(
    pool: web::Data<DbPool>,
    //web::Query(rd): web::Query<RoundData>,//GET Param Type
    web::Form(rd): web::Form<RoundData>,//URL Encoded Form
) -> Result<HttpResponse, Error> {
    let conn = pool.get().expect("couldn't get db connection from pool");
    //cloning steam to new value cause for whatever reason using it moves the whole rd struct -.-
    
    println!("Received Data:\nsteam: {}, win: {}, points: {}, tp: {}, tc: {}", rd.steam_id, rd.did_win, rd.round_points, rd.team_points, rd.team_count);

    let steam_id = rd.steam_id.clone();
    // fetch user data from the steam id in non-blocking thread
    let user = web::block(move || sfdb_connect::find_user_by_steam(&conn, steam_id))
        .await
        .map_err(|e| {
            eprintln!("{}", e);
            HttpResponse::InternalServerError().finish()
        })?;
    if let Some(user) = user {
        //calculate new RWS value using previous values and new points
        if let Some(newRws) = rws::calculate_rws(
            user.rws,
            user.rounds_total as f32,
            rd.did_win,
            rd.round_points as f32,
            rd.team_points as f32,
            rd.team_count as f32,
        ) {
            println!("Got new RWS {}", newRws);

            // gets a second connection from the pool since the other was moved to the other thread.
            // TODO: alternatively we could just also grab this data in that thread.... maybe i'll change this later
            let conn2 = pool.get().expect("couldn't get db connection 2 from pool");
            // update user in db with new RWS score (rounds gets incremented automatically by this too)
            let user =
                web::block(move || sfdb_connect::update_newround_user_by_id(&conn2, user.id, newRws))
                    .await
                    .map_err(|e| {
                        eprintln!("{}", e);
                        HttpResponse::InternalServerError().finish()
                    })?;
            if let Some(user) = user {
                Ok(HttpResponse::Ok().json(user))
            } else {
                println!("Failed updating stats for user with steamid: {}", rd.steam_id);
                Ok(HttpResponse::NotFound().body(format!("Failed updating stats for user with steamid: {}", rd.steam_id)))
            }
        } else {
            println!("There was an issue calculating RWS for user with steamid: {}\nErrored Stats: RWS{},RT{},W{},RP{},TP{},TC{}", rd.steam_id, user.rws,user.rounds_total,rd.did_win,rd.round_points,rd.team_points,rd.team_count);
            Ok(HttpResponse::BadRequest().body(format!("Invalid values submitted for user with steamid: {}", rd.steam_id)))
        }
    } else {
        println!("No user found with steamid: {}", rd.steam_id);
        Ok(HttpResponse::NotFound().body(format!("No user found with steamid: {}", rd.steam_id)))
    }

}

// vip api: /vip/steam/<steamid> gets vip status via steamid
//todo

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    // Init ENV Logging
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    if let Some(action) = std::env::args().nth(1) {
        let res = match action.to_lowercase().as_str() {
            "start" => start().await,
            "stop" => stop(),
            "monitor" => {
                Ok(())
            },
            "help" => {
                println!("Arg Commands:\nstart   | starts the server\nstop    | stops the server\nmonitor | checks the server status and reboots it if it crashed or is unresponsive");
                Ok(())
            },
            _ => {
                panic!("Invalid command argument! Availible include: start, stop, monitor");
            },
        };
        return res;
    } else {
        panic!("No argument supplied! Commands include: start, stop, monitor");
    }
}

async fn start() -> std::io::Result<()> {
    // set up database connection pool
    let pool = establish_connection_pool();

    // create http authorization check
    //let auth = HttpAuthentication::basic(validator);

    HttpServer::new(move || {
        App::new() //with_state(AppState { db: addr.clone() })
            .data(pool.clone())
            // enable logger
            .wrap(middleware::Logger::default())
            .service(
                web::scope("/v1/") //there is no need to have /api/ scope since NGINX is going to be redirecting us under /api anyways
                    .service(post_user_init)
                    .service(web::scope("/id/").service(stats_get_by_id))
                    //.service(get_top)//gets top x ammount players /stats/top/{count}
                    .service(post_new_round),
            )
        //.service(web::resource("/index.html").to(|| async { "Hello world!" }))
        .service(web::resource("/").to(check))
    })
    .bind("127.0.0.1:1337")?
    .run()
    .await
}

fn stop() -> std::io::Result<()> {
    Ok(())
}

//TODO: Update tests to reliably test current API functions
#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::dev::Service;
    use actix_web::{http, test, web, App, Error};

    #[actix_rt::test]
    async fn test_index() -> Result<(), Error> {
        let app = App::new().route("/", web::get().to(index));
        let mut app = test::init_service(app).await;

        let req = test::TestRequest::get().uri("/").to_request();
        let resp = app.call(req).await.unwrap();

        assert_eq!(resp.status(), http::StatusCode::OK);

        let response_body = match resp.response().body().as_ref() {
            Some(actix_web::body::Body::Bytes(bytes)) => bytes,
            _ => panic!("Response error"),
        };

        assert_eq!(response_body, r##"Hello world!"##);

        Ok(())
    }
}
