use std::fmt::Error;
use std::fs;
use actix_web::{web, App, HttpResponse, HttpServer};
use actix_web::web::Path;
use serde::Deserialize;
use serde_json::json;
use crate::model::database::posts;
use crate::model::posts_database::{create_new_post_database, delete_post_database, update_post_database};

pub async fn get_new_post() -> HttpResponse {
        let mut handlebars= handlebars::Handlebars::new();
    let index_template = fs::read_to_string("templates/new_post.hbs").unwrap();
    handlebars
         .register_template_string("new_post", &index_template).expect("TODO: panic message");

    let html = handlebars.render("new_post", &json!({"o":"ax"})).unwrap() ;
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html)

}
pub async fn receive_new_posts(form: web::Form<posts>) -> HttpResponse
{
    let mut handlebars= handlebars::Handlebars::new();
    let index_template = fs::read_to_string("templates/message_display.hbs").unwrap();
    handlebars
        .register_template_string("message_display", &index_template).expect("TODO: panic message");


    let title=&form.title;
    let description=&form.description;
    let name=&form.name;

    create_new_post_database(title, description, name).await;
    let success_message="the post created successfully";
    let html = handlebars.render("message_display", &json!({"message":success_message})).unwrap() ;

    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html)

}


pub async fn delete_post(to_be_deleted_post: web::Path<String> )->HttpResponse
    {
    println!("ads");
let to_delete=&to_be_deleted_post.into_inner();

println!("------->{}", to_delete);

    delete_post_database(to_delete).await.expect(" panic message");
    let mut handlebars= handlebars::Handlebars::new();
    let index_template = fs::read_to_string("templates/message_display.hbs").unwrap();
    handlebars
        .register_template_string("message_display", &index_template).expect("TODO: panic message");
    let success_message="the post deleted successfully";
    let html = handlebars.render("message_display", &json!({"message":success_message})).unwrap() ;

    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html)



}

pub async fn  page_to_update_post(to_be_updated_post: web::Path<String> )->HttpResponse{

    let mut handlebars= handlebars::Handlebars::new();
    let index_template = fs::read_to_string("templates/update_post.hbs").unwrap();
    handlebars
        .register_template_string("update_post", &index_template).expect("TODO: panic message");

   let to_be_updated_post=to_be_updated_post.into_inner();
   println!("🤩🤩🤩🤩🤩{:?}", to_be_updated_post);

    update_post_helper(to_be_updated_post).await;
//Todo should send the current post title to the next page
    let html = handlebars.render("update_post", &json!({"o":"ax"})).unwrap() ;
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html)

}

pub async fn update_post_helper(ids: String) -> String {
    ids
}
pub async fn receive_updated_post(form: web::Form<posts>, id: web::Path<String>) ->HttpResponse
{

    //todo get the data from the url form post method
    let mut handlebars= handlebars::Handlebars::new();
    let index_template = fs::read_to_string("templates/message_display.hbs").unwrap();
    handlebars
        .register_template_string("message_display", &index_template).expect("TODO: panic message");

let current_post_name= id.clone();
    let title=&form.title;
    let description=&form.description;
    let name=&form.name;
println!("------------------------------>{}", title);

    update_post_database(title, description, name,current_post_name).await.expect("TODO: panic message");
    let success_message="the post created successfully";
    let html = handlebars.render("message_display", &json!({"message":success_message})).unwrap() ;

    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html)

}