use std::fmt::Error;

use actix_web::{web, HttpResponse, Result, ResponseError};
use serde::Deserialize;
use crate::model::database::{posts, select_posts};

#[derive(Deserialize)]
pub struct PaginationParams {
    page: Option<i32>,
    per_page: Option<i32>,
}

#[derive(Deserialize)]
pub struct Total_pages{
    total_pages_count: Option<i32>,
}


use actix_web::{ App, Error as ActixError, HttpServer};
use futures::TryFutureExt;
use sqlx::postgres::{PgPoolOptions, PgRow};
use sqlx::Row;

#[derive(Debug)]
pub struct MyError {
    error: ActixError,
}

impl std::convert::From<ActixError> for MyError {
    fn from(error: ActixError) -> Self {
        Self { error }
    }
}

impl std::fmt::Display for MyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "An error occurred: \"{}\"",
            self.error.to_string()
        ))
    }
}

impl ResponseError for MyError {
    fn status_code(&self) -> actix_web::http::StatusCode {
        self.error.as_response_error().status_code()
    }



}
pub fn paginate<T>(items: Vec<T>, page: i32, per_page: i32) -> Vec<T> {
    let start_index = (page - 1) * per_page;
    let end_index = start_index + per_page;
    items.into_iter().skip(start_index as usize).take(per_page as usize).collect()
}

//
// pub async  fn  get_count_of_posts(x:i32) ->i32 {
//     println!("{:?}",x);
//     let v=x as i32;
//   //  static mut n: i32 = v;
//     let total_pages_count= x  as i32;
//     println!("😊😊😊😊😊😊{:?}",total_pages_count);
//
//     total_pages_count
//     //println!("{:?}",n.to_string());
//
// }

//pub async fn get_users(params: web::Path<String>) -> Result<HttpResponse,MyError> {
pub async fn pagination_logic(params: web::Query<PaginationParams>  ) -> Result<Vec<posts>,MyError>
{

    let page = params.page.unwrap_or(1);
    let per_page = params.per_page.unwrap_or(3);


    let mut posts_pagination:Vec<posts>= select_posts().await.expect("maosdso");
    let paginated_users = paginate(posts_pagination.clone(), page, per_page);


    let posts_per_page_length = posts_pagination.len();
    println!("😀😀😀😀😀😀😀{:?}", &posts_per_page_length);
     // let response = HttpResponse::Ok().json(paginated_users);
    Ok(paginated_users)
  //  Ok(())
}

//
// fn getss_users(params: web::Query<PaginationParams>) -> Result<HttpResponse> {
//     let users = vec!["Alice", "Bob", "Charlie", "David", "Eve", "Frank", "Grace", "Heidi", "Ivan", "Judy"];
//     let page = params.page.unwrap_or(1);
//     let per_page = params.per_page.unwrap_or(5);
//     let paginated_users = paginate(users, page, per_page);
//
//
//     let response = HttpResponse::Ok().json(paginated_users);
//     Ok(response)
// }


