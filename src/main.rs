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
use crate::controller::category_controller::{category_controller, delete_category, get_new_category, receive_new_category};
use crate::controller::pagination_controller::{pagination_display, perfect_pagination_logic};
use crate::controller::posts_controller::{delete_post, get_new_post, page_to_update_post, receive_new_posts, receive_updated_post};
use crate::controller::single_post_controller::get_single_post;
use crate::model::database::{select_all_from_table, select_specific_pages_post};
use crate::model::pagination_database::{ pagination_logic};


// async fn index(req: HttpRequest)->Responder<Body=()> {
//      println!("🏏🏏🏏🏏");
// }

#[tokio::main]
async fn main() -> Result<()>{


     HttpServer::new(|| {
          App::new()

              .service(web::resource("/").to(get_all_posts))
              .service(web::resource("/categories/{name}").to(category_controller))
              .service(web::resource("/post_specific/{title}").to(get_single_post))
              .service(web::resource("/users").to(pagination_display))


//posts
              .service(web::resource("/posts").to(pagination_display))
              .service(web::resource("/posts/new").to(get_new_post))
              .service(web::resource("/posts").route(web::post().to(receive_new_posts)))
              .service(web::resource("/posts/{title}").to(get_single_post))
              .service(web::resource("/posts/{title}/edit").route(web::get().to(page_to_update_post)))

              // Todo change delete_post to the delete method
             .service(web::resource("/delete_post/{title}").route(web::get().to(delete_post)))
              .service(web::resource("/posts/{title}").route(web::post().to(receive_updated_post)))
              // .service(web::resource("/posts/{title}").route(web::delete().to(receive_updated_post)))


              // category
//todo create a route /category get all the categories

              .service(web::resource("/category/new").to(get_new_category))
                  .service(web::resource("/category").route(web::post().to(receive_new_category)))
              // Todo change delete_post to the delete method and url to --> /category/{name}
              .service(web::resource("/delete_category/{name}").route(web::get().to(delete_category)))
              .service(web::resource("/posts/{title}").route(web::post().to(receive_updated_post)))
              .service(web::resource("/category/{title}/edit").route(web::get().to(page_to_update_post)))


     })
         .bind("127.0.0.1:8080")?
         .run().await.expect("TODO: panic message");
     Ok(())
}
