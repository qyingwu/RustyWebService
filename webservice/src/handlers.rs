use super::state::AppState;
use actix_web::{web, HttpResponse};
use crate::db_access::get_courses_for_teacher_db;
use crate::db_access::post_new_course_db;
use crate::db_access::get_course_details_db;
use super::errors::MyError;


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


pub async fn new_course(
    new_course: web::Json<Course>, // Automatically deserializes JSON request body into Course struct
    app_state: web::Data<AppState>, // Shared application state containing DB pool
) -> HttpResponse {
    //calling async function use await, new_course is extractor
    // post_new_course_db is an async DB operation that:
    // 1. Takes a reference to DB pool (&app_state.db)
    // 2. Takes ownership of Course data (new_course.into())
    // 3. Returns the created course
    let course = post_new_course_db(&app_state.db, new_course.into()).await;
    HttpResponse::Ok().json(course)
}

pub async fn get_courses_for_teacher(
    app_state: web::Data<AppState>,
    params: web::Path<(i32,)>,
) -> Result<HttpResponse, MyError> {
    let teacher_id = params.into_inner().0;
    get_courses_for_teacher_db(&app_state.db, teacher_id)
    .await
    .map(|courses| HttpResponse::Ok().json(courses))
}

pub async fn get_course_detail(
    app_state: web::Data<AppState>,
    params:web::Path<(i32, i32)>,
) -> HttpResponse {
    let teacher_id = i32::try_from(params.0).unwrap();
    let course_id = i32::try_from(params.1).unwrap();
    let course = get_course_details_db(&app_state.db, teacher_id, course_id).await;
    HttpResponse::Ok().json(course)
}


#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::http::StatusCode;
    use std::sync::Mutex;
    use dotenv::dotenv;
    use sqlx::postgres::PgPoolOptions;
    use std::env;
    use std::time::Duration;

    //because the function is async, need to use actix_rt for async test
    #[actix_rt::test]
    async fn post_course_test() {
        println!("Current directory: {:?}", std::env::current_dir().unwrap());
        dotenv().ok();

        let db_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set.");
        let db_pool = PgPoolOptions::new().connect(&db_url).await.unwrap();

        //create request app_state
        let app_state: web::Data<AppState> = web::Data::new(AppState {
            health_check_response: "".to_string(),
            visit_count: Mutex::new(0),
            db:db_pool,
        });

        let course = web::Json(Course {
            teacher_id: 1,
            name: "test course".into(),
            id: None,
            time: None,
        });

        let resp = new_course(course, app_state).await;
        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[actix_rt::test]
    async fn get_all_courses_success() {
        //std::env::set_var("DATABASE_URL", "postgres://username:password@localhost:5432/database_name");
        dotenv().ok();

        let db_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set.");
        let db_pool = PgPoolOptions::new().connect(&db_url).await.unwrap();


        let app_state: web::Data<AppState> = web::Data::new(AppState {
            health_check_response: "".to_string(),
            visit_count: Mutex::new(0),
            db:db_pool,
        });

        let teacher_id: web::Path<(i32,)> = web::Path::from((1,));
        let resp = get_courses_for_teacher(app_state, teacher_id).await.unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[actix_rt::test]
    async fn get_one_course_success() {
        //std::env::set_var("DATABASE_URL", "postgres://username:password@localhost:5432/database_name");
        dotenv().ok();

        let db_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set.");
        let db_pool = PgPoolOptions::new().connect(&db_url).await.unwrap();


        let app_state: web::Data<AppState> = web::Data::new(AppState {
            health_check_response: "".to_string(),
            visit_count: Mutex::new(0),
            db:db_pool,
        });
        let params: web::Path<(i32, i32)> = web::Path::from((1, 1));
        let resp = get_course_detail(app_state, params).await;
        assert_eq!(resp.status(), StatusCode::OK);
    }
}
