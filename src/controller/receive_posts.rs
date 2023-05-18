use std::fs;
use actix_web::{web, App, HttpResponse, HttpServer};
use serde::Deserialize;
use serde_json::json;
use crate::model::create_new_posts_database::create_new_post_database;
use crate::model::database::posts;


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
pub async fn index(form: web::Form<posts>) -> HttpResponse
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

