use actix_cors::Cors;
use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};

#[get("/hello")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world")
}

#[get("/api/v1//package/search")]
async fn search_package() -> impl Responder {
    HttpResponse::Ok().body(include_str!("../mock-data/search_sbom.json"))
}
#[get("/api/v1/config")]
async fn get_config() -> impl Responder {
    let content = include_str!("../mock-data/config.json");
    HttpResponse::Ok().body(content)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .wrap(Cors::permissive())
            .service(search_package)
            .service(hello)
            .service(get_config)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind(("127.0.0.1", 8025))?
    .run()
    .await
}
