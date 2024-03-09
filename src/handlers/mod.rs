mod handlers;

use async_trait::async_trait;

pub use handlers::create_question;
pub use handlers::get_questions;
use rocket::serde::json::Json;
use rocket_db_pools::Connection;

use crate::StackoverflowDb;

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
    async fn get_questions(conn: Connection<StackoverflowDb>) -> Result<String, String>;
}
