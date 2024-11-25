use actix_web::{web, App, HttpServer};
use std::io;
use std::sync::Mutex;
use dotenv::dotenv;
use std::env;
use sqlx::postgres::PgPoolOptions;


#[path = "../handlers.rs"]
mod handlers;
#[path = "../routers.rs"]
mod routers;
#[path = "../state.rs"]
mod state;
#[path = "../models.rs"]
mod models;

use routers::*;
use state::AppState;

#[actix_rt::main]
async fn main() -> io::Result<()> {
    dotenv().ok();

    //panic in case not able to read url
    let databse_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set.");
    let db_pool = PgPoolOptions::new().connect(&databse_url).await.unwrap();


    let shared_data = web::Data::new(AppState {
        health_check_response: "I'm OK.".to_string(),
        visit_count: Mutex::new(0),
        //courses: Mutex::new(vec![]),
        db: db_pool,
    });

    //move forces the closure to take ownership of 
    //any variables it uses from the enclosing scope
    //||  is a closure (anonymous function) syntax in Rust, 
    //similar to lambda functions in other languages
//// Without move (might not work if shared_data needs to outlive the current scope) => borrowing
//The closure is passed to HttpServer::new() which will use 
// it across multiple threads
//Without move, the closure would try to borrow shared_data, 
//which isn't safe across thread boundaries
    let app = move || {
        App::new()
        .app_data(shared_data.clone()) // owns shared_data
        .configure(general_routes)
        .configure(course_routes)
    };

    HttpServer::new(app).bind("127.0.0.1:3000")?.run().await
}