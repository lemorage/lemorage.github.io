use crate::components;
use actix_web::{HttpResponse, Result};
use components::{footer, header, meta_tags};
use maud::{html, Markup, DOCTYPE};

pub async fn about() -> Result<HttpResponse> {
    let bio = "Glenn Miao is a developer specializing in RAG Agents...";

    let rendered: Markup = html! {
        (DOCTYPE)
        html lang="en" {
            head {
                meta charset="UTF-8";
                meta name="viewport" content="width=device-width, initial-scale=1.0";
                title { "About Me | Glenn Miao" }
                (meta_tags::meta_tags())
            }
            body class="light" {
                (header::header("about"))
                div id="sycamore-root" {}
                div class="container" {
                    h1 { "About Me" }
                    p { (bio) }
                    a href="/" { "Back to Home" }
                }
                (footer::footer())
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
