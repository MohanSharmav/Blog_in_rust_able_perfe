mod model;
mod controller;

use std::env::Args;
use std::fmt::{Debug, Error, Formatter};
use std::future::Future;
use std::io::Read;
use std::path::Path;
use sqlx::postgres::PgPoolOptions;
use actix_files::NamedFile;
use actix_web::{App, HttpRequest, HttpResponse, HttpServer, Responder, Result, web};
use actix_web::http::StatusCode;
use tokio::select;
use warp::reply::with_status;
use controller::home_page::get_all_posts;
use model::database::selecting;
use warp::{get, Rejection, Reply};
use crate::controller::category_controller::{category_controller};
use crate::controller::pagination_controller::{pagination_display, perfect_pagination_logic};
use crate::controller::receive_posts::{get_new_post, index};
use crate::controller::single_post_controller::get_single_post;
use crate::model::database::{select_all_from_table, select_specific_pages_post};
use crate::model::pagination_database::{ pagination_logic};


// async fn index(req: HttpRequest)->Responder<Body=()> {
//      println!("🏏🏏🏏🏏");
// }

#[tokio::main]
async fn main() -> Result<()>{

//test start
   //  count_posts().await;
   //   select_specific_pages_post().await;
  //   get_users("1".to_string()).await.expect("asdsdssd");
//
// selecting().await.expect("TODO: panic message");
//      select_all_from_table().await.expect("paamnaic message");
//      //test end
     HttpServer::new(|| {
          App::new()

              .service(web::resource("/ben").to(get_all_posts))
              .service(web::resource("/categories/{name}").to(category_controller))
              .service(web::resource("/posts/{title}").to(get_single_post))
              .service(web::resource("/users").to(pagination_display))
              .service(web::resource("/new_posts").to(get_new_post))
         //   .service(web::resource("/recieved").route(web::post().to(index)))
     //        .service(web::resource("/is-admin").route(web::post().to(set_is_admin)))
     })
         .bind("127.0.0.1:8080")?
         .run().await.expect("TODO: panic message");
     Ok(())
}
