mod handlers;

use async_trait::async_trait;
use rocket::serde::json::Json;
use rocket_db_pools::Connection;

use crate::StackoverflowDb;
pub use handlers::{create_question, get_questions};

use self::handlers::Question;

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
    async fn get_questions(conn: Connection<StackoverflowDb>) -> Result<Vec<Question>, String>;
}
