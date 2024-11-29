use actix_web::{HttpResponse, Result};
use maud::{html, Markup, DOCTYPE};

pub async fn not_found() -> Result<HttpResponse> {
    let rendered: Markup = html! {
        (DOCTYPE)
        html lang="en" {
            head {
                meta charset="UTF-8";
                meta name="viewport" content="width=device-width, initial-scale=1.0";
                title { "404 | Page Not Found" }
                link rel="stylesheet" href="/static/style.css";
            }
            body class="light" {
                div class="not-found-container" {
                    h1 { "404" }
                    p { "Sorry, we couldn’t find that page." }
                    a href="/" { "Go back to home" }
                }
            }
        }
    };

    Ok(HttpResponse::NotFound()
        .content_type("text/html")
        .body(rendered.into_string()))
}
