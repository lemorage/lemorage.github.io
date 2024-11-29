mod about;
mod components;
mod home;

use actix_files::Files;
use actix_web::{web, App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(move || {
        App::new()
            .route("/", web::get().to(home::index))
            .route("/about", web::get().to(about::about))
            .service(Files::new("/static", "static"))
    })
    .bind("0.0.0.0:7536")?
    .run()
    .await
}
