use std::arch::asm;
use actix_web::guard::Post;
use serde::Serialize;
use sqlx::{Error, Pool, Postgres, Row};
use sqlx::postgres::{PgPoolOptions, PgRow};
use serde::Deserialize;
use sqlx::FromRow;

#[derive(Deserialize)]
#[derive(Debug, Clone, PartialEq,Serialize)]
pub struct categories {
    pub(crate) name: String,
}

#[derive(Deserialize)]
#[derive(Debug, Clone, PartialEq,Serialize,sqlx::FromRow)]
pub struct posts{
    pub(crate) title: String,
    pub(crate) description: String,
    pub(crate) name: String,
}

#[derive(Deserialize)]
#[derive(Debug, Clone, PartialEq,Serialize,sqlx::FromRow)]
pub struct update_post{
    pub(crate) current_title: String,
    pub(crate) title: String,
    pub(crate) description: String,
    pub(crate) name: String,
}




pub(crate) async fn selecting() ->Result<Vec<String>, ()>{


    dotenv::dotenv().expect("Unable to load environment variables from .env file");

    let db_url = std::env::var("DATABASE_URL").expect("Unable to read DATABASE_URL env var");

    let mut pool = PgPoolOptions::new()
        .max_connections(100)
        .connect(&db_url)
        .await.expect("Unable to connect to Postgres");


let mut vect=Vec::new();
    let  rows = sqlx::query("SELECT name FROM categories")
        .fetch_all(&pool)
        .await.expect("Unable to");

    for row in rows{
        let names: String=row.get("name");

      //  let original_Array =Foo { name: names.to_string() };

        vect.push(names);

    }


    Ok(vect)
}




pub async fn select_all_from_table() -> Result<Vec<String>,Error> {

    dotenv::dotenv().expect("Unable to load environment variables from .env file");

    let db_url = std::env::var("DATABASE_URL").expect("Unable to read DATABASE_URL env var");

    let mut pool = PgPoolOptions::new()
        .max_connections(100)
        .connect(&db_url)
        .await.expect("Unable to connect to Postgres");

    let mut all_posts = Vec::new();


    let rows = sqlx::query("SELECT title,description,name FROM posts")
          .fetch_all(&pool)
          .await?;
    for row in rows {
        let title: String = row.get("title");
        let description: String = row.get("description");
        let name: String = row.get("name");
        let all_posts_string= title+" " +&*description +" "+ &*name;
       // let all_posts_string=format!(title, description, name);
     //   let all_posts_json = posts { title: title.to_string(), description: description.to_string(), name: name.to_string() };
        all_posts.push(all_posts_string);
    }

    let  x:i32= all_posts.len() as i32;
    println!("xxxxxxxx {:?}",x);

//let all_posts_json=serde_json::to_string(&all_posts).expect("noooooo");
    Ok(all_posts)
}

pub async fn select_posts()->Result<Vec<posts>,Error>
{
    dotenv::dotenv().expect("Unable to load environment variables from .env file");

    let db_url = std::env::var("DATABASE_URL").expect("Unable to read DATABASE_URL env var");

    let mut pool = PgPoolOptions::new()
        .max_connections(100)
        .connect(&db_url)
        .await.expect("Unable to connect to Postgres");


    let mut postsing = sqlx::query_as::<_, posts>("select title, description, name from posts")
        .fetch_all(&pool)
        .await
        .unwrap();

    Ok(postsing)
}

//new function for selecting specific post with pointers
pub async fn select_specific_pages_post(start_page: &Option<i32>) ->Result<Vec<posts>,Error>
{
 let mut start_page= start_page.unwrap();

//    let end_posts_count ;
    let end_posts_count = start_page+3;
    if(start_page==1)
    {
        let end_posts_count = 3;
    }
        println!("⭐️{}---{}",start_page,end_posts_count);

    // if(start_page>1)
    // {
    //     start_page=start_page+2
    // }
    // println!("⭐️{}---{}",start_page,start_page*3);
println!("🐒{:?}",start_page);
    dotenv::dotenv().expect("Unable to load environment variables from .env file");

    let db_url = std::env::var("DATABASE_URL").expect("Unable to read DATABASE_URL env var");
   // println!("{}{}", start_page,start_page*3);

    let mut pool = PgPoolOptions::new()
        .max_connections(100)
        .connect(&db_url)
        .await.expect("Unable to connect to Postgres");

    let mut perfect_posts = sqlx::query_as::<_, posts>("select * from posts where post_id between $1 and $2")
        .bind(start_page)
        .bind(start_page+2)
        .fetch_all(&pool)
        .await
        .unwrap();

    println!("🐶{:?}",perfect_posts);
    Ok(perfect_posts)
}