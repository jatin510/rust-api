mod handlers;

pub use handlers::create_question;
pub use handlers::get_question;

pub enum HandlerError {
    BadRequest(String),
    InternalError(String),
}

pub trait THandler {
    fn create_question() -> Result<String, String>;
    fn get_question() -> Result<String, String>;
}
