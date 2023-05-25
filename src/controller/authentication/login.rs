use std::fmt::Error;
use std::fs;
use actix_web::{HttpResponse, web};
use serde::Deserialize;
use serde_json::json;

use actix_identity::{Identity, IdentityMiddleware};
use actix_session::{config::PersistentSession, storage::CookieSessionStore, SessionMiddleware};
use actix_web::{
    cookie::{time::Duration, Key},
    error,
    http::StatusCode,
    middleware, App, HttpMessage as _, HttpRequest, HttpServer, Responder,
};

#[derive(Debug, Clone, PartialEq,Deserialize)]
pub struct user{
    username: String,
    password: String
}
pub async fn get_login_page() -> HttpResponse {
    println!("Welcome to login page");
    let mut handlebars = handlebars::Handlebars::new();
    let index_template = fs::read_to_string("templates/login.hbs").unwrap();
    handlebars
        .register_template_string("login", &index_template).expect("TODO: panic message");


    let html = handlebars.render("login", &json!({"yy":"uuihiuhuihiuhuih"})).unwrap();


    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html)
}

pub async fn get_data_from_login_page(form: web::Form<user>,    req: HttpRequest) -> HttpResponse
{
println!("🦋");

 let user = &form.username;
    let password=&form.password;

    println!("{}", user);

    let mut handlebars= handlebars::Handlebars::new();
    let index_template = fs::read_to_string("templates/message_display.hbs").unwrap();
    handlebars
        .register_template_string("message_display", &index_template).expect("TODO: panic message");





    Identity::login(&req.extensions(), user.to_string()).unwrap();

    // web::Redirect::to("/").using_status_code(StatusCode::FOUND);


    let success_message="user successfully authenticated";
    let html = handlebars.render("message_display", &json!({"message":success_message})).unwrap() ;


    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html)
}



pub async fn logout(id: Identity) -> impl Responder {
    id.logout();

    web::Redirect::to("/").using_status_code(StatusCode::FOUND)
}