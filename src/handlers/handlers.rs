use rocket_db_pools::sqlx::{self};

use crate::StackoverflowDb;
use async_trait::async_trait;
use rocket::serde::{json::Json, Deserialize};
use rocket_db_pools::sqlx::Row;
use rocket_db_pools::Connection;
use uuid::Uuid;

use super::THandler;

#[derive(Debug, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Question {
    #[serde(skip_deserializing)]
    pub id: Option<i32>,
    pub title: String,
    pub body: String,
}

#[post("/questions", data = "<question_json>")]
pub async fn create_question(
    question_json: Json<Question>,
    mut db: Connection<StackoverflowDb>,
) -> Result<String, String> {
    println!("create question");

    let id = Uuid::new_v4().to_string();

    let query = sqlx::query("INSERT INTO questions (id, title, body) VALUES (?, ?, ?)")
        .bind(id)
        .bind(question_json.title.clone())
        .bind(question_json.body.clone())
        .execute(&mut **db)
        .await;

    match query {
        Ok(query) => {
            println!("success: {:?}", query);
            return Ok("Question is created successfully".to_string());
        }
        Err(err) => {
            println!("error: {:?}", err);
            return Err("Error creating Question".to_string());
        }
    }
}

#[get("/questions")]
pub async fn get_questions(mut db: Connection<StackoverflowDb>) -> Result<String, String> {
    println!("get question api");
    // println!("{:?}", conn);
    let query = sqlx::query("SELECT * FROM test ")
        .fetch_one(&mut **db)
        .await
        .and_then(|r| Ok(r.try_get(0)?))
        .ok();

    let result = query.unwrap();
    return Result::Ok(result);
}

pub struct Handler {}

#[async_trait]
impl THandler for Handler {
    async fn create_question(
        question_json: Json<Question>,
        conn: Connection<StackoverflowDb>,
    ) -> Result<String, String> {
        println!("inside create question handler");
        return create_question(question_json, conn).await;
    }

    async fn get_questions(conn: Connection<StackoverflowDb>) -> Result<String, String> {
        return get_questions(conn).await;
    }
}
