#[macro_use]
extern crate diesel;
extern crate dotenv;

//actix-web
use actix_web::{http, middleware, web, App, HttpRequest, HttpResponse, HttpServer, get, post, Error};
use serde::{Serialize, Deserialize};

use sfdb_connect::models::*;
use diesel::prelude::*;
use sfdb_connect::*;

use diesel::r2d2::{self, ConnectionManager};
//type DbPool = r2d2::Pool<ConnectionManager<MysqlConnection>>;


// struct AppState {
//     db: Addr<DbExecutor>,
// }


async fn index(req: HttpRequest) -> &'static str {
    println!("REQ: {:?}", req);
    "Hello world!"
}

#[derive(Deserialize)]
pub struct AuthRequest {
   name: String
}

async fn stream_auth(req: HttpRequest, web::Query(info): web::Query<AuthRequest>) -> HttpResponse {
    println!("received stream name param: {:?}", info.name);
    HttpResponse::TemporaryRedirect().header("Location", "fetched-username-from-key-here").body("AUTHENTICATION SUCCESS")
}

// ranks api: /ranks (set,get,put,post)
// /stats/{userid}
// part of the ranks api. Returns stats for given steam id
#[get("/stats/{id}")]
async fn stats_get(pool: web::Data<DbPool>, id: web::Path<i32>) -> Result<HttpResponse, Error> {
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
        let res = HttpResponse::NotFound()
            .body(format!("No user found with id: {}", id));
        Ok(res)
    }
}


async fn ranks_post() -> &'static str {
    "yote"
}

async fn ranks_put() -> &'static str {
    "heyooo"
}


// vip api: /vip/steam/<steamid> gets vip status via steamid

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    // Init ENV Logging
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    // Init our auth to paddle's api
    //let jwt = match external::get_jwt(&api_key, &api_secret) {
    //    Ok(v) => v,
    //    Err(e) => panic!("Could not get the JWT: {}", e),
    //};

    // Start 3 parallel db executors
    // let addr = SyncArbiter::start(3, || {
    //     DbExecutor(establish_connection())
    // });

    // set up database connection pool
    // let connspec = std::env::var("DATABASE_URL").expect("DATABASE_URL");
    // let manager = ConnectionManager::<MysqlConnection>::new(connspec);
    // let pool = r2d2::Pool::builder()
    //     .build(manager)
    //     .expect("Failed to create pool.");
    let pool = establish_connection_pool();

    HttpServer::new(move || {
        
        App::new()//with_state(AppState { db: addr.clone() })
            .data(pool.clone())
            // enable logger
            .wrap(middleware::Logger::default())
            .service(
                web::scope("/v1/")
                .service(stats_get)
             )

            //.resource("/{name}", |r| r.method(Method::GET).a(db_index))
            //.service(web::resource("/index.html").to(|| async { "Hello world!" }))
            //.service(web::resource("/").to(index))
            //.service(web::resource("/ranks").route(web::get().to(ranks_get)))
            //.service(web::resource("/on_publish").route(web::get().to(stream_auth)))
    })
    .bind("127.0.0.1:1337")?
    .run()
    .await
}

/// Async handler
// async fn db_index(req: &HttpRequest<AppState>) -> HttpResponse {
//     let name = &req.match_info()["name"];

//     // Send message to `DbExecutor` actor
//     req.state()
//         .db
//         .send(NewUser {
//             display_name: &name.to_owned(),
//             steamid2: "dank",
//         })
//         .from_err()
//         .and_then(|res| match res {
//             Ok(user) => Ok(HttpResponse::Ok().json(user)),
//             Err(_) => Ok(HttpResponse::InternalServerError().into()),
//         })
//         .responder()
// }

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