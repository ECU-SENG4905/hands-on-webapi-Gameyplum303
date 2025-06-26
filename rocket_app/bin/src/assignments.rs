use rocket::{serde::json::Json, State, get, post, put, delete};
use diesel::r2d2::{self, ConnectionManager};
use diesel::sqlite::SqliteConnection;
use tasks_db_lib::models::{UserTask, NewUserTask};
use tasks_db_lib::crud::CrudOperations;

pub type DbPool = r2d2::Pool<ConnectionManager<SqliteConnection>>;

#[derive(rocket::serde::Deserialize)]
pub struct AssignmentInput {
    pub user_id: i32,
    pub task_id: i32,
    pub task_status_id: i32,
}

#[get("/assignments")]
pub async fn get_user_tasks(pool: &State<DbPool>) -> Json<Vec<UserTask>> {
    let mut conn = pool.get().expect("db connection");
    let assignments = UserTask::read_all(&mut conn).unwrap_or_default();
    Json(assignments)
}

#[get("/assignments/<user_id>/<task_id>")]
pub async fn get_user_task(user_id: i32, task_id: i32, pool: &State<DbPool>) -> Option<Json<UserTask>> {
    let mut conn = pool.get().ok()?;
    UserTask::read(&mut conn, (user_id, task_id)).ok().flatten().map(Json)
}

#[post("/assignments", data = "<assignment>")]
pub async fn create_user_task(pool: &State<DbPool>, assignment: Json<AssignmentInput>) -> Option<Json<UserTask>> {
    let mut conn = pool.get().ok()?;
    let new_assignment = NewUserTask {
        user_id: assignment.user_id,
        task_id: assignment.task_id,
        task_status_id: assignment.task_status_id,
    };
    UserTask::create(&mut conn, new_assignment).ok().map(Json)
}

#[put("/assignments/<user_id>/<task_id>", data = "<assignment>")]
pub async fn update_user_task(user_id: i32, task_id: i32, pool: &State<DbPool>, assignment: Json<AssignmentInput>) -> Option<Json<UserTask>> {
    let mut conn = pool.get().ok()?;
    let updated_assignment = NewUserTask {
        user_id: assignment.user_id,
        task_id: assignment.task_id,
        task_status_id: assignment.task_status_id,
    };
    UserTask::update(&mut conn, (user_id, task_id), updated_assignment).ok().map(Json)
}

#[delete("/assignments/<user_id>/<task_id>")]
pub async fn delete_user_task(user_id: i32, task_id: i32, pool: &State<DbPool>) -> Option<Json<usize>> {
    let mut conn = pool.get().ok()?;

    match UserTask::delete(&mut conn, (user_id, task_id)) {
        Ok(count) => Some(Json(count)),
        Err(e) => {
            eprintln!("Delete error: {}", e);
            Some(Json(0))
        }
    }
}