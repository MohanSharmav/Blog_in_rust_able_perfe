use std::fmt::Error;
use std::fs;
use actix_web::{HttpResponse, web};
use serde_json::json;
use crate::model::category_database::category_controller_database_function;

pub async fn category_controller(path: web::Path<String>)->HttpResponse
{


    let mut category_input = path.into_inner();
    println!("😊😊😊😊😊😊 comee on{:?}",category_input);
    println!("😊😊😊😊😊😊-------------{:?}",category_input);
    let mut handlebars= handlebars::Handlebars::new();
    let index_template = fs::read_to_string("templates/category.hbs").unwrap();
    handlebars
        .register_template_string("category", &index_template).expect("TODO: panic message");

    let category_postinng=category_controller_database_function(category_input).await.expect("TODO: panic message");

    println!(" 😋  😋  😋 {:?}",category_postinng);
    let html = handlebars.render("category", &json!({"p":&category_postinng})).unwrap() ;

    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html)
}