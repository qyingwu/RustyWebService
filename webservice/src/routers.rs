use super::handlers::*;
use actix_web::web;

pub fn general_routes(cfg: &mut web::ServiceConfig) {
    cfg.route("/health", web::get().to(health_check_handler));
}

/**
Creates a function that configures routes for course-related endpoints
POST localhost:3000/courses/
The scope /courses groups all course-related endpoints
add more routes like:
GET /courses/ (list courses)
GET /courses/{id} (get specific course)
PUT /courses/{id} (update course)
**/
pub fn course_routes(cfg: &mut web::ServiceConfig) {
    cfg
    .service(web::scope("/courses")
    .route("/", web::post().to(new_course)));
}