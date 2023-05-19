use std::fmt::Error;
use std::fs;
use actix_web::{HttpResponse, web};
use serde_json::json;
use crate::model::category_database::{category_controller_database_function, create_new_category_database, delete_category_database};
use crate::model::database::categories;

pub async fn category_controller(path: web::Path<String>)->HttpResponse
{


    let mut category_input = path.into_inner();
    // println!("😊😊😊😊😊😊 comee on{:?}",category_input);
    // println!("😊😊😊😊😊😊-------------{:?}",category_input);
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


pub async fn get_new_category() -> HttpResponse {
    let mut handlebars= handlebars::Handlebars::new();
    let index_template = fs::read_to_string("templates/new_category.hbs").unwrap();
    handlebars
        .register_template_string("new_category", &index_template).expect("TODO: panic message");

    println!("😇😇😇😇😇😇😇");
    let html = handlebars.render("new_category", &json!({"o":"ax"})).unwrap() ;
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html)
}

pub async fn receive_new_category(form: web::Form<categories>) -> HttpResponse
{
    println!("🥳🥳🥳🥳🥳🥳🥳🥳🥳🥳🥳🥳");
    let mut handlebars= handlebars::Handlebars::new();
    let index_template = fs::read_to_string("templates/message_display.hbs").unwrap();
    handlebars
        .register_template_string("message_display", &index_template).expect("TODO: panic message");


println!("🔥🔥🔥🔥🔥🔥🔥🔥");
    let name=&form.name;
    println!("------------------->{}", name);

    create_new_category_database(name).await.expect("TODO: panic message");
    let success_message="the categories created successfully";
    let html = handlebars.render("message_display", &json!({"message":success_message})).unwrap() ;

    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html)

}

pub async fn delete_category(id: web::Path<categories>) -> HttpResponse
{

    println!("ad🥳s🥳🥳🥳🥳🥳🥳🥳🥳🥳");
    let to_delete_category=&id.name;

    println!("------->{}", to_delete_category);

    delete_category_database(to_delete_category).await.expect(" panic message");
    let mut handlebars= handlebars::Handlebars::new();
    let index_template = fs::read_to_string("templates/message_display.hbs").unwrap();
    handlebars
        .register_template_string("message_display", &index_template).expect("TODO: panic message");
    let success_message="the category deleted successfully";
    let html = handlebars.render("message_display", &json!({"message":success_message})).unwrap() ;

    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html)

}