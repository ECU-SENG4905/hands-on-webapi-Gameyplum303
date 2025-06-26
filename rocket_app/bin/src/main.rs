mod users;
use users::*;
mod tasks;
mod statuses;
mod assignments;

use rocket::{launch, routes, Build, Rocket};
use diesel::r2d2::{self, ConnectionManager};
use diesel::sqlite::SqliteConnection;
use tasks::*;
use statuses::*;
use assignments::*;

pub type DbPool = r2d2::Pool<ConnectionManager<SqliteConnection>>;

#[launch]
async fn rocket() -> Rocket<Build> {
    dotenvy::dotenv().ok();
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<SqliteConnection>::new(database_url);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");

    rocket::build()
        .manage(pool)
        .mount(
            "/api",
            routes![
                get_users, get_user, create_user, update_user, delete_user 
            ],
        )
}
