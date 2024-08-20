use actix_files as fs;
use actix_web::{web, App, HttpServer, Responder};

async fn index() -> impl Responder {
    "API is running"
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/api", web::get().to(index))
            .service(fs::Files::new("/", "./static").index_file("index.html"))
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
