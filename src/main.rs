mod model;
mod controller;

use std::env::Args;
use std::fmt::{Debug, Error, Formatter};
use std::future::Future;
use std::io::Read;
use std::path::Path;
use sqlx::postgres::PgPoolOptions;
use actix_files::NamedFile;
use actix_web::{App, HttpRequest, HttpResponse, HttpServer, Result, web};
use actix_web::http::StatusCode;
use tokio::select;
use warp::reply::with_status;
use controller::home_page::get_all_posts;
use model::database::selecting;
use warp::{get, Rejection, Reply};
use crate::controller::category_controller::{category_controller};
use crate::controller::pagination_controller::{pagination_display, perfect_pagination_logic};
use crate::controller::single_post_controller::get_single_post;
use crate::model::database::{select_all_from_table};
use crate::model::pagination_database::{ pagination_logic};


async fn index(req: HttpRequest)-> Result<NamedFile>{
     let path= Path::new("templates/index.hbs");
     Ok(NamedFile::open(path)?)
}

#[tokio::main]
async fn main() -> Result<()>{

//test start
   //  count_posts().await;
     get_all_posts().await;
     perfect_pagination_logic().await;
  //   get_users("1".to_string()).await.expect("asdsdssd");
     
selecting().await.expect("TODO: panic message");
     select_all_from_table().await.expect("paamnaic message");
     //test end 
     HttpServer::new(|| {
          App::new()
              .service(web::resource("/").to(get_all_posts))
              .service(web::resource("/categories/{name}").to(category_controller))
              .service(web::resource("/posts/{title}").to(get_single_post))
              .service(web::resource("/users").to(pagination_display))
     })
         .bind("127.0.0.1:8080")?
         .run().await.expect("TODO: panic message");
     Ok(())
}
