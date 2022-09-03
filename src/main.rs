use actix_web::{middleware, web, App, HttpServer, Result, HttpResponse};
use serde::Serialize;

#[derive(Serialize)]
struct Ping {
    message: String,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .configure(routes)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/v1")
            .service(web::resource("/ping").to(ping)),
    );
}

async fn ping() -> HttpResponse {
    HttpResponse::Ok().json(Ping {
        message: "pong".to_string(),
    })
}
