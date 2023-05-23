use std::fmt::Error;
use sqlx::postgres::PgPoolOptions;
use crate::controller::posts_controller::update_post_helper;
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

pub async fn  delete_post_database(to_delete: String) ->Result<(),Error>
{
    dotenv::dotenv().expect("Unable to load environment variables from .env file");

    let db_url = std::env::var("DATABASE_URL").expect("Unable to read DATABASE_URL env var");

    let mut pool = PgPoolOptions::new()
        .max_connections(100)
        .connect(&db_url)
        .await.expect("Unable to connect to Postgres");

    let to_delete =to_delete;

    sqlx::query("delete from posts where title =$1")
        .bind(to_delete)
        .execute(&pool)
        .await
        .expect("Unable toasdasd");
println!("Successfully deleted");
    Ok(())
}

pub async fn update_post_database(title: &String, description: &String, name: &String, current_post_name: &String) ->Result<(),Error>{
    dotenv::dotenv().expect("Unable to load environment variables from .env file");

    let db_url = std::env::var("DATABASE_URL").expect("Unable to read DATABASE_URL env var");

    let mut pool = PgPoolOptions::new()
        .max_connections(100)
        .connect(&db_url)
        .await.expect("Unable to connect to Postgres");
    // UPDATE Customers
    // SET ContactName = 'Alfred Schmidt', City= 'Frankfurt'
    // WHERE CustomerID = 1;
    sqlx::query("update posts set title=$1 ,description=$2,name=$3 where title=$4")
        .bind(title)
        .bind(description)
        .bind(name)
        .bind(current_post_name)
        .execute(&pool)
        .await
        .expect("Unable toasdasd");
    Ok(())
}