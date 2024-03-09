mod handlers;

use async_trait::async_trait;
use rocket::serde::{json::Json, Deserialize};
use rocket_db_pools::Connection;
use serde::Serialize;

use crate::StackoverflowDb;
pub use handlers::{create_question, get_questions};

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(crate = "rocket::serde")]
pub struct Question {
    #[serde(skip_deserializing)]
    pub id: Option<String>,
    pub title: String,
    pub body: String,
}

pub enum HandlerError {
    BadRequest(String),
    InternalError(String),
}

#[async_trait]
pub trait THandler {
    async fn create_question(
        question_json: Json<Question>,
        conn: Connection<StackoverflowDb>,
    ) -> Result<String, String>;
    async fn get_questions(
        conn: Connection<StackoverflowDb>,
    ) -> Result<Json<Vec<Question>>, String>;
}
