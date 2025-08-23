use crate::components;
use crate::config::Config;
use actix_web::{HttpResponse, Result};
use components::{footer, meta_tags};
use maud::{html, Markup, DOCTYPE};

pub async fn index() -> Result<HttpResponse> {
    let name = "Glenn Miao";
    let description: Markup = html! {
        span {
            "Dev @ "
            a href=(Config::CURRENT_URL) target="_blank" rel="noopener noreferrer" {
                "Flowith AI"
            }
            br;
            "Open Source Developer"
            br;
            "Previously Founding Software Engineer at "
            a href=(Config::PREVIOUS_URL) target="_blank" rel="noopener noreferrer" {
                "Wondervoy"
            }
        }
    };

    let rendered: Markup = html! {
        (DOCTYPE)
        html lang="en" {
            head {
                meta charset="UTF-8";
                meta name="viewport" content="width=device-width, initial-scale=1.0";
                title { "Home | Glenn Miao" }
                link rel="preload" href=(Config::AVATAR_URL) as="image";
                (meta_tags::meta_tags())
            }
            body class="light" {
                div id="sycamore-root" {}

                div class="container" {
                    div class="avatar" {
                        img src=(Config::AVATAR_URL) alt="Main Profile Picture" class="front";
                        img src=(Config::HOVER_AVATAR_URL) alt="Virtual Profile Picture" class="back" loading="lazy";
                    }
                    h1 { (name) }
                    p { (description) }
                    div class="links" {
                        a href="/about" { "About" }
                        " | "
                        a href=(Config::BLOG_URL) { "Blog" }
                        " | "
                        a href=(Config::GITHUB_URL) { "Portfolio" }
                        " | "
                        a href="#" onclick="confirmContact();" { "Contact" }
                    }
                }
                (footer::footer())
            }

            (maud::PreEscaped(format!(r#"
                <script type="module">
                    import init from '/static/frontend/wasm_frontend.js';
                    init();
                </script>
                <script>
                    function confirmContact() {{
                        if (confirm("Do you want to send an email?")) {{
                            window.location.href = "mailto:{}?subject=Ti%20Amo&body=Hi%20there,%0D%0A%0D%0A";
                        }}
                    }}
                </script>
            "#, Config::EMAIL)))
        }
    };

    Ok(HttpResponse::Ok()
        .content_type("text/html")
        .body(rendered.into_string()))
}
