use sqlx::Error;
use sqlx::postgres::PgPoolOptions;
use crate::model::database::posts;

pub async fn category_controller_database_function(category:String)->Result<Vec<posts>,Error>
{

println!(" inn database--------   - --{}", category);

    dotenv::dotenv().expect("Unable to load environment variables from .env file");

    let db_url = std::env::var("DATABASE_URL").expect("Unable to read DATABASE_URL env var");

    let mut pool = PgPoolOptions::new()
        .max_connections(100)
        .connect(&db_url)
        .await.expect("Unable to connect to Postgres");


    let mut category_posts = sqlx::query_as::<_, posts>("select title, description,name from posts where name=$1")
         .bind(category)
        .fetch_all(&pool)
        .await.expect("Unable to get");

    println!(" 😋  😋  😋 {:?}",category_posts);

    Ok(category_posts)
}