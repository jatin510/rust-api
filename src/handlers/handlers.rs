use rocket_db_pools::sqlx::{self};

use async_trait::async_trait;

use crate::StackoverflowDb;
use rocket_db_pools::sqlx::Row;
use rocket_db_pools::Connection;

use super::THandler;

#[post("/question")]
pub async fn create_question(mut db: Connection<StackoverflowDb>) -> Result<String, String> {
    println!("create question");
    // println!("{:?}", conn);
    return Result::Ok("create question".to_string());
}

#[get("/question")]
pub async fn get_question(mut db: Connection<StackoverflowDb>) -> Result<String, String> {
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
    async fn create_question(mut conn: Connection<StackoverflowDb>) -> Result<String, String> {
        return create_question(conn).await;
    }

    async fn get_question(mut conn: Connection<StackoverflowDb>) -> Result<String, String> {
        return get_question(conn).await;
    }
}
