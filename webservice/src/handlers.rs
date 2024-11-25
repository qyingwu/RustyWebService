use super::state::AppState;
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

use super::models::Course;
use chrono::Utc;

pub async fn new_course(
    new_course: web::Json<Course>,
    app_state: web::Data<AppState>,
) -> HttpResponse {
    println!("Received new course");
    let course_count = app_state
        .courses
        .lock()
        .unwrap()
        .clone()
        .into_iter()
        .filter(|course| course.teacher_id == new_course.teacher_id)
        .collect::<Vec<Course>>()
        .len();
    
    let new_course = Course {
        teacher_id: new_course.teacher_id,
        id: Some(course_count + 1),
        name: new_course.name.clone(),
        time: Some(Utc::now().naive_utc()),
    };
    app_state.courses.lock().unwrap().push(new_course);
    HttpResponse::Ok().json("Course added")
}


#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::http::StatusCode;
    use std::sync::Mutex;

    //because the function is async, need to use actix_rt for async test
    #[actix_rt::test]
    async fn post_course_test() {
        //create request
        let course = web::Json(Course {
            teacher_id: 1,
            name: "test course".into(),
            id: None,
            time: None,
        });
        
        let app_state: web::Data<AppState> = web::Data::new(AppState {
            health_check_response: "".to_string(),
            visit_count: Mutex::new(0),
            courses:Mutex::new(vec![]),
        });
        let resp = new_course(course, app_state).await;
        assert_eq!(resp.status(), StatusCode::OK);
    }
    
}