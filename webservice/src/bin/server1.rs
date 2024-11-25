use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use std::io;

//configure route, http get
pub fn general_routes(cfg: &mut web::ServiceConfig) {
    cfg.route("/health", web::get().to(health_check_handler));
}

//configure handler
pub async fn health_check_handler() -> impl Responder {
    HttpResponse::Ok().json("Actix Web Service is running!")
}

//run http server
#[actix_rt::main]
async fn main() -> io::Result<()> {
    //create app, configure route
    let app = move || {
        App::new()
            .app_data(web::Data::new(()))
            .configure(general_routes)
    };

    //run httpserver
    HttpServer::new(app)
        .bind("127.0.0.1:3000")?
        .run()
        .await
}