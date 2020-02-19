//! Actix web Diesel integration example
//!
//! Diesel does not support tokio, so we have to run it in separate threads using the web::block
//! function which offloads blocking code (like Diesel's) in order to not block the server's thread.

#[macro_use]
extern crate diesel;

use actix_web::{get, middleware, post, web, App, Error, HttpResponse, HttpServer};
// use diesel::prelude::*;
use diesel::mysql::MysqlConnection;
use diesel::r2d2::{self, ConnectionManager};
use uuid::Uuid;

mod models;
mod actions;

use models::models::{NewUser};

type DbPool = r2d2::Pool<ConnectionManager<MysqlConnection>>;

/// Finds user by UID.
#[get("/user/{user_id}")]
async fn get_user(
    pool: web::Data<DbPool>,
    user_uid: web::Path<Uuid>,
) -> Result<HttpResponse, Error> {
    let user_uid = user_uid.into_inner();
    let conn = pool.get().expect("couldn't get db connection from pool");

    // use web::block to offload blocking Diesel code without blocking server thread
    let user = web::block(move || actions::find_user_by_uid(user_uid, &conn))
        .await
        .map_err(|e| {
            eprintln!("{}", e);
            HttpResponse::InternalServerError().finish()
        })?;

    if let Some(user) = user {
        Ok(HttpResponse::Ok().json(user))
    } else {
        let res = HttpResponse::NotFound()
            .body(format!("No user found with uid: {}", user_uid));
        Ok(res)
    }
}

/// Inserts new user with name defined in form.
#[post("/user")]
async fn add_user(
    pool: web::Data<DbPool>,
    form: web::Json<NewUser>
) -> Result<HttpResponse, Error> {
    println!("In post");

    let conn = pool.get().expect("couldn't get db connection from pool");

   // use web::block to offload blocking Diesel code without blocking server thread
    let user = web::block(move || actions::insert_new_user(&form.name, &conn))
        .await
        .map_err(|e| {
            eprintln!("{}", e);
            HttpResponse::InternalServerError().finish()
        })?;

    Ok(HttpResponse::Ok().json(user))
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_BACKTRACE", "1");
    std::env::set_var("RUST_LOG", "my_errors=debug,actix_web=info,diesel=debug");

    env_logger::init();
    dotenv::dotenv().ok();

    // set up database connection pool
    let connspec = std::env::var("DATABASE_URL").expect("DATABASE_URL");
    let manager = ConnectionManager::<MysqlConnection>::new(connspec);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");

    let bind = "127.0.0.1:8080";

    println!("Starting server at: {}", &bind);

    // Start HTTP server
    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .wrap(middleware::Logger::default())
            .service(get_user)
            .service(add_user)
    })
    .bind(&bind)?
    .run()
    .await
}