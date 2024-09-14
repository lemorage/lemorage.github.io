use actix_files::Files;
use actix_web::{web, App, HttpResponse, HttpServer, Result};
use maud::{html, Markup, DOCTYPE};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(move || {
        App::new()
            .route("/", web::get().to(index))
            .route("/about", web::get().to(about))
            .service(Files::new("/static", "static"))
    })
    .bind("127.0.0.1:7536")?
    .run()
    .await
}

async fn index() -> Result<HttpResponse> {
    let avatar_url =
        "https://gravatar.com/avatar/f596c7140610305ef8414aa73c7a1db3?size=256&cache=1725778928828";
    let hover_avatar_url =
        "https://gravatar.com/userimage/215159183/321f49eca2a5a689000be949787e1e45.jpeg?size=256";
    let name = "Glenn Miao";
    let description =
        "Building RAG Agents<br>Previously @SUSTech<br>Founding AI Engineer of @Wondervoy";
    let blog_url = "#";
    let portfolio_url = "#";

    let rendered: Markup = html! {
        (DOCTYPE)
        html lang="en" {
            head {
                meta charset="UTF-8";
                meta name="viewport" content="width=device-width, initial-scale=1.0";
                title { "Home | Glenn Miao" }
                link rel="stylesheet" href="/static/style.css";
                link rel="preload" href=(avatar_url) as="image";
                link rel="apple-touch-icon" sizes="180x180" href="/static/apple-touch-icon.png";
                link rel="icon" type="image/png" sizes="32x32" href="/static/favicon-32x32.png";
                link rel="icon" type="image/png" sizes="16x16" href="/static/favicon-16x16.png";
                link rel="icon" type="image/png" sizes="192x192" href="/static/android-chrome-192x192.png";
                link rel="icon" type="image/png" sizes="512x512" href="/static/android-chrome-512x512.png";
                link rel="shortcut icon" href="/static/favicon.ico";
                link rel="stylesheet" href="https://cdnjs.cloudflare.com/ajax/libs/font-awesome/6.0.0-beta3/css/all.min.css";
            }
            body class="light" {
                div id="sycamore-root" {}

                div class="container" {
                    div class="avatar" {
                        img src=(avatar_url) alt="Main Profile Picture" class="front";
                        img src=(hover_avatar_url) alt="Virtual Profile Picture" class="back" loading="lazy";
                    }
                    h1 { (name) }
                    p { (maud::PreEscaped(description)) }
                    div class="links" {
                        a href=(blog_url) { "Blog" }
                        " | "
                        a href=(portfolio_url) { "Portfolio" }
                        " | "
                        a href="/about" { "About Me" }
                        " | "
                        a href="mailto:one.lemorage@gmail.com?subject=Ti%20Amo&body=Hi%20there,%0D%0A%0D%0A" { "Contact" }
                    }
                }
                div class="footer-main" {
                    "Glenn Miao " (maud::PreEscaped("&copy;")) " 2024. All rights reserved."
                }
            }

            (maud::PreEscaped(r#"
                <script type="module">
                    import init from '/static/frontend/frontend.js';
                    init();
                </script>
            "#))
        }
    };

    Ok(HttpResponse::Ok()
        .content_type("text/html")
        .body(rendered.into_string()))
}

async fn about() -> Result<HttpResponse> {
    let bio = "Glenn Miao is a developer specializing in RAG Agents...";

    let rendered: Markup = html! {
        (DOCTYPE)
        html lang="en" {
            head {
                meta charset="UTF-8";
                meta name="viewport" content="width=device-width, initial-scale=1.0";
                title { "About Me | Glenn Miao" }
                link rel="stylesheet" href="/static/style.css";
            }
            body class="light" {
                div class="container" {
                    h1 { "About Me" }
                    p { (bio) }
                    a href="/" { "Back to Home" }
                }
                div class="footer-main" {
                    "Glenn Miao " (maud::PreEscaped("&copy;")) " 2024. All rights reserved."
                }
            }
        }
    };

    Ok(HttpResponse::Ok()
        .content_type("text/html")
        .body(rendered.into_string()))
}
