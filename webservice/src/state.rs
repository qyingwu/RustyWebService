use std::sync::Mutex;
use super::models::Course;

pub struct AppState {
    //health_check_response shared within multile threads, not mut
    pub health_check_response: String,
    pub visit_count: Mutex<u32>,
    pub courses: Mutex<Vec<Course>>
}
