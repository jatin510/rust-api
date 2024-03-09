#[macro_use]
extern crate rocket;

mod handlers;

use rocket_db_pools::sqlx::{self};
use rocket_db_pools::Database;

#[derive(Database)]
#[database("stackoverflow")]
pub struct StackoverflowDb(sqlx::SqlitePool);

use handlers::{create_question, get_questions};

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(StackoverflowDb::init())
        .mount("/", routes![create_question, get_questions,])
}
