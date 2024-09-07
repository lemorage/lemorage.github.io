use actix_files::Files;
use actix_web::{web, App, HttpResponse, HttpServer, Result};
use tera::{Context, Tera};

async fn index(tmpl: web::Data<Tera>) -> Result<HttpResponse> {
    let mut ctx = Context::new();
    ctx.insert("avatar_url", "https://gravatar.com/avatar/460cd9f43cc848b0da104aaae7d6e1e73b9c6f7f896c67e1e8e906ba2ec66e02");
    ctx.insert("name", "Glenn Miao");
    ctx.insert(
        "description",
        "Building RAG Agents<br>Previously @SUSTech<br>Founding AI Engineer of @Wondervoy",
    );
    ctx.insert("blog_url", "#");
    ctx.insert("portfolio_url", "#");
    ctx.insert("contact_url", "#");

    let rendered = tmpl
        .render("index.html", &ctx)
        .map_err(|_| actix_web::error::ErrorInternalServerError("Template Error"))?;
    Ok(HttpResponse::Ok().content_type("text/html").body(rendered))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let tera = Tera::new(concat!(env!("CARGO_MANIFEST_DIR"), "/templates/**/*")).unwrap();

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(tera.clone()))
            .route("/", web::get().to(index))
            .service(Files::new("/static", "static"))
    })
    .bind("127.0.0.1:7536")?
    .run()
    .await
}
