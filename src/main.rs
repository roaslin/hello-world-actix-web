use std::sync::Mutex;

use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

struct AppState {
    app_name: String,
    counter: Mutex<i32>,
}

#[get("/staticappstate")]
async fn static_app_state(data: web::Data<AppState>) -> impl Responder {
    let app_name = &data.app_name;

    format!("Hello {app_name}!")
}

#[get("/mutableappstate")]
async fn mutable_app_state(data: web::Data<AppState>) -> impl Responder {
    let mut counter = data.counter.lock().unwrap();
    *counter += 1;

    format!("Counter is {counter}!")
}

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

#[shuttle_runtime::main]
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let app_state = web::Data::new(AppState {
        app_name: String::from("Static App State!!"),
        counter: Mutex::new(0),
    });

    HttpServer::new(move || {
        let app_state_scope = web::scope("/appstate")
            .service(static_app_state)
            .service(mutable_app_state);

        App::new()
            .app_data(app_state.clone())
            .service(hello)
            .service(echo)
            .service(app_state_scope)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
