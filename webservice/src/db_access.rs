use super::models::*;
use sqlx::postgres::PgPool;

pub async fn get_courses_for_teacher_db(pool: &PgPool, teacher_id:i32) -> Vec<Course> {
    //$ means var teacher_id
    
    let rows = sqlx::query!(
        r#"SELECT 
        id, 
        teacher_id "teacher_id!: i32", 
        name as "name!: String",
        time
        FROM course
        WHERE teacher_id = $1"#,
        teacher_id
    )
    .fetch_all(pool) //check multiple 
    .await.unwrap();

    // $1 is a PostgreSQL parameter placeholder
    // The actual value that replaces $1

    //map course 
    rows.iter()
        .map(|r| Course {
            id: Some(r.id),
            teacher_id: r.teacher_id,
            name: r.name.clone(),
            time: r.time.map(|t| t.naive_utc()),  // Convert DateTime<Utc> to NaiveDateTime
        })
        .collect()
}


pub async fn get_course_details_db(pool: &PgPool, teacher_id: i32, course_id: i32) -> Option<Course> {
    let row = sqlx::query!(
        r#"SELECT 
        id, 
        teacher_id as "teacher_id!: i32",
        name as "name!: String",
        time
        FROM course
        WHERE teacher_id = $1 and id = $2"#,
        teacher_id,
        course_id
    )
    .fetch_one(pool)
    .await
    .ok()?;

    Some(Course {
        id: Some(row.id),
        teacher_id: row.teacher_id,
        name: row.name.clone(),
        time: row.time.map(|t| t.naive_utc()),
    })
}


pub async fn post_new_course_db(pool: &PgPool, new_course: Course) -> Course {
    let row = sqlx::query!(
        r#"INSERT INTO course (teacher_id, name)
        VALUES ($1, $2)
        RETURNING id as "id!: i32", teacher_id as "teacher_id!: i32", name as "name!: String", time"#,
        new_course.teacher_id,
        new_course.name
    )
    .fetch_one(pool)
    .await.unwrap();

    Course{
        id: Some(row.id),
        teacher_id: row.teacher_id,
        name: row.name.clone(),
        time: row.time.map(|t| t.naive_utc()), 
    }

}