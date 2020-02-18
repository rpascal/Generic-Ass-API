#[macro_use]
extern crate diesel;

pub mod models;
pub mod schema; // Ignore errors from this for now; it doesn't get created unti later

use actix_web::{web, App, HttpRequest, HttpServer, Responder};
use listenfd::ListenFd;
// mod db;

use dotenv::dotenv;
use diesel::prelude::*;
use std::env;



#[derive(Queryable)]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub published: i8,
}

pub fn establish_connection() -> MysqlConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    MysqlConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}


#[actix_rt::main]
async fn main() -> std::io::Result<()> {

    use self::schema::posts::dsl::*;

    let connection = establish_connection();
    let results = posts.filter(published.eq(1))
        .limit(5)
        .load::<Post>(&connection)
        .expect("Error loading posts");

    println!("Displaying {} posts", results.len());
    for post in results {
        println!("{}", post.title);
        println!("----------\n");
        println!("{}", post.body);
    }

    let mut listenfd = ListenFd::from_env();
    let mut server = HttpServer::new(|| App::new().route("/", web::get().to(index)));

    server = if let Some(l) = listenfd.take_tcp_listener(0).unwrap() {
        server.listen(l)?
    } else {
        server.bind("127.0.0.1:8088")?
    };

    server.run().await
}

async fn index(_req: HttpRequest) -> impl Responder {
    "Hello World!"
}