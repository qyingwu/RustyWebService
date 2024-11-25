use std::sync::Mutex;
//use super::models::Course;
use sqlx::postgres::PgPool;

//can be used in multi threaded env
pub struct AppState {
    //health_check_response shared within multile threads, not mut
    pub health_check_response: String,
    pub visit_count: Mutex<u32>,
    //pub courses: Mutex<Vec<Course>>
    pub db: PgPool,
}
