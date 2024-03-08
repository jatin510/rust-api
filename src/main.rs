#[macro_use]
extern crate rocket;

mod handlers;

use rocket_db_pools::sqlx::{self, Row};
use rocket_db_pools::{Connection, Database};

#[derive(Database)]
#[database("stackoverflow")]
pub struct StackoverflowDb(sqlx::SqlitePool);

use handlers::{create_question, get_question};

#[get("/questions")]
pub async fn get_questions(mut db: Connection<StackoverflowDb>) -> Option<String> {
    println!("get questions api");

    let query = sqlx::query("SELECT content FROM stackoverflow WHERE id = ?")
        .fetch_one(&mut **db)
        .await
        .and_then(|r| Ok(r.try_get(0)?))
        .ok();

    return query;
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(StackoverflowDb::init())
        .mount("/", routes![create_question, get_question, get_questions])
}
