use crate::handlers::{course::*, general::*};
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
    .route("/", web::post().to(post_new_course))
    .route("/{teacher_id}", web::get().to(get_courses_for_teacher))
    .route("/{teacher_id}/{course_id}", web::get().to(get_course_detail))
    .route("/{teacher_id}/{course_id}", web::delete().to(delete_course))
    .route("/{teacher_id}/{course_id}", web::put().to(update_course_details)));
}