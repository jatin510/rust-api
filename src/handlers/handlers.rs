use rocket_db_pools::sqlx::{self};

use super::THandler;

#[post("/question")]
pub fn create_question() -> Result<String, String> {
    println!("create question");
    return Result::Ok("create question".to_string());
}

#[get("/question")]
pub fn get_question() -> Result<String, String> {
    println!("get question api");

    // let query = sqlx::query("SELECT content FROM logs WHERE id = ?")
    //     .fetch_one(&mut **db)
    //     .await
    //     .and_then(|r| Ok(r.try_get(0)?))
    //     .ok();

    return Result::Ok("get question".to_string());
}

pub struct Handler {}

impl THandler for Handler {
    fn create_question() -> Result<String, String> {
        return create_question();
    }

    fn get_question() -> Result<String, String> {
        return get_question();
    }
}
