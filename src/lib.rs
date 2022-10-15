use actix_web::{web, App, HttpServer, Responder, HttpResponse};
use actix_web::dev::Server;

async fn health_check() -> impl Responder {
    HttpResponse::Ok().finish()
}

pub fn run() -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| {
        App::new()
            .route("/health", web::get().to(health_check))
    })
        .bind("127.0.0.1:8000")?
        .run();

    Ok(server)
}