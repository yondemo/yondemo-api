use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
mod domain_models;
mod operations;
mod repositories;

use crate::domain_models::User;

use crate::repositories::UserRepository;
use crate::operations::*;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[get("/stub_some_user")]
async fn users_stub_some() -> impl Responder {
    let manager = manage::user::UserManager { repository: UserRepository::StubSome };
    match manager.get() {
        None => {
            HttpResponse::Ok().body("")
        }
        Some(user) => {
            let body = format!("{}", user.nickname());
            HttpResponse::Ok().body(body)
        }
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let _user = User::new("mary");
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(echo)
            .service(users_stub_some)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
