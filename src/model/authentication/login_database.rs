use sqlx::Error;
use sqlx::postgres::PgPoolOptions;

pub async fn login_database(user: &String, password: &String) -> i32
{
    dotenv::dotenv().expect("Unable to load environment variables from .env file");

    let db_url = std::env::var("DATABASE_URL").expect("Unable to read DATABASE_URL env var");

    let mut pool = PgPoolOptions::new()
        .max_connections(100)
        .connect(&db_url)
        .await.expect("Unable to connect to Postgres");

  let v=  sqlx::query("select count(1) from users where name=$1 AND password=$2")
        .bind(user)
        .bind(password)
        .fetch_all(&pool)
        .await.expect("unable to fetch the user");

    println!("successfully logged in");
    let user=user.to_string();
    let password = password.to_string();
v.get(0)
}