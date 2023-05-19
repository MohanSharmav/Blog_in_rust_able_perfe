use std::fmt::Error;
use std::fs;
use actix_web::{web, App, HttpResponse, HttpServer};
use serde::Deserialize;
use serde_json::json;
use crate::model::database::posts;
use crate::model::posts_database::{create_new_post_database, delete_post_database};


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


pub async fn delete_post( id: web::Path<String>)->HttpResponse
{
    println!("ads");
let to_delete=&id.into_inner();

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