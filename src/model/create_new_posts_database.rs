use std::fmt::Error;
use sqlx::postgres::PgPoolOptions;
use crate::model::database::posts;

pub async fn create_new_post_database(title: &String, description: &String, name:  &String) -> Result<(),Error> {
    dotenv::dotenv().expect("Unable to load environment variables from .env file");

    let db_url = std::env::var("DATABASE_URL").expect("Unable to read DATABASE_URL env var");

    let mut pool = PgPoolOptions::new()
        .max_connections(100)
        .connect(&db_url)
        .await.expect("Unable to connect to Postgres");
    sqlx::query("insert into posts(title,description,name) values ($1,$2,$3)")
        .bind(title)
        .bind(description)
        .bind(name)
        .execute(&pool)
        .await
        .expect("Unable toasdasd");


    Ok(())

}