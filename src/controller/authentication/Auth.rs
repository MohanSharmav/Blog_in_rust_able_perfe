// use actix_web::{
//     web,
//     App,
//     HttpResponse,
//     HttpServer,
//     Responder,
// };
//
// use actix_identity::{Identity, IdentityMiddleware};
//
// #[derive(Debug)]
// pub struct User {
//     username: String,
//     password: String,
// }
//
// impl User {
//     fn new(username: String, password: String) -> Self {
//         Self { username, password }
//     }
// }
//
// pub async fn index(user: Identity) -> impl Responder {
//     HttpResponse::Ok().body(format!("{}", user.identity()))
// }
//
// pub async fn login(user: web::Data<User>) -> impl Responder {
//     if user.username == "admin" && user.password == "password" {
//         HttpResponse::Ok().body("Login successful")
//     } else {
//         HttpResponse::Unauthorized().body("Invalid username or password")
//     }
// }
//
// pub async fn logout(user: Identity) -> impl Responder {
//     user.forget();
//     HttpResponse::Ok().body("Logged out")
// }
//
