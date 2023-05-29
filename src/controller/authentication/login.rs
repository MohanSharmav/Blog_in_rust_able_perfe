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
use crate::model::authentication::login_database::login_database;


//extra
use argon2::password_hash::rand_core::OsRng;
use argon2::password_hash::SaltString;
use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier};


#[derive(Debug, Clone, PartialEq,Deserialize)]
pub struct user{
    pub(crate) username: String,
    pub(crate) password: String
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

    //
    // let stored_password = match &user.password {
    //     None => return HttpResponse::BadRequest().body("Invalid username or password"), // NOTE: login as tremporary user is not allowed
    //     Some(password) => password,
    // };
    //
    // let stored_hash = PasswordHash::new("asd-asd").unwrap();
    // let pw_valid = Argon2::default()
    //     .verify_password(pw.as_bytes(), &stored_hash)
    //     .is_ok();


let x=login_database(user, password).await;





if(x==1) {

    Identity::login(&req.extensions(), user.to_string()).unwrap();

    let success_message="user successfully authenticated";
    let html = handlebars.render("message_display", &json!({"message":success_message})).unwrap() ;


    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html)
}else{
     HttpResponse::BadRequest().body("Invalid email or password")

}

}



pub async fn logout(id: Identity) -> impl Responder {
    id.logout();


    web::Redirect::to("/").using_status_code(StatusCode::FOUND)
}