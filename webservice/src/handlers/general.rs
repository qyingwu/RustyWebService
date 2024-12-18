use crate::state::AppState;
use actix_web::{web, HttpResponse};

pub async fn health_check_handler(
    app_state: web::Data<AppState>
) -> HttpResponse {
    //health_check_response shared, not mutable
    let health_check_response = &app_state.health_check_response;
    //mut value, this value needs to be locked before accessing
    let mut visit_count = app_state.visit_count.lock().unwrap();
    let response = 
        format!("{} {} times", health_check_response, visit_count);
    //* is dereferencing this MutexGuard 
    //to access and modify the actual integer value inside
    //Without the *, you'd be trying to add 1 to the 
    //MutexGuard itself, not the value it protects
    *visit_count += 1;
    //after the handler, the lock will be released
    HttpResponse::Ok().json(&response)
}