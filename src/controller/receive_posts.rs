use std::fs;
use actix_web::{web, App, HttpResponse, HttpServer};
use serde::Deserialize;
use serde_json::json;

#[derive(Deserialize)]
pub struct FormData {
    username: String,
}

pub async fn get_new_post() -> HttpResponse {
        let mut handlebars= handlebars::Handlebars::new();
    let index_template = fs::read_to_string("templates/new_post.hbs").unwrap();
    handlebars
         .register_template_string("new_post", &index_template).expect("TODO: panic message");
// println!("ADADADSsad");
//
//     let x=&form.username;
// println!("ads asd");
//     println!("{:?}",x);
    let html = handlebars.render("new_post", &json!({"o":"asdadsax"})).unwrap() ;
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html)

}
pub async fn index(form: web::Form<FormData>) -> HttpResponse {
//println!("ADSSADDASD");
//     let mut handlebars= handlebars::Handlebars::new();
//     let index_template = fs::read_to_string("templates/new_post.hbs").unwrap();
//     handlebars
//         .register_template_string("new_post", &index_template).expect("TODO: panic message");
// println!("ADADADSsad");



    println!("{}", form.username);
  //  HttpResponse::Ok().body(format!("username: {}", form.username))

    HttpResponse::Ok().body(format!("{}",form.username))

    // let html = handlebars.render("single", &json!({"o":"ADSs"})).unwrap() ;
    // HttpResponse::Ok()
    //     .content_type("text/html; charset=utf-8")
    //     .body(html)

}

