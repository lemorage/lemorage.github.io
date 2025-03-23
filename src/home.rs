use crate::components;
use actix_web::{HttpResponse, Result};
use components::{footer, meta_tags};
use maud::{html, Markup, DOCTYPE};

pub async fn index() -> Result<HttpResponse> {
    let avatar_url =
        "https://gravatar.com/avatar/f596c7140610305ef8414aa73c7a1db3?size=256&cache=1725778928828";
    let hover_avatar_url =
        "https://gravatar.com/userimage/215159183/321f49eca2a5a689000be949787e1e45.jpeg?size=256";
    let name = "Glenn Miao";
    let description =
        "Building RAG Agents<br>Previously @SUSTech<br>Founding AI Engineer of @Wondervoy";
    let blog_url = "https://lemorage.gitlab.io/lemorage-blog";
    let portfolio_url = "https://github.com/lemorage";

    let rendered: Markup = html! {
        (DOCTYPE)
        html lang="en" {
            head {
                meta charset="UTF-8";
                meta name="viewport" content="width=device-width, initial-scale=1.0";
                title { "Home | Glenn Miao" }
                link rel="preload" href=(avatar_url) as="image";
                (meta_tags::meta_tags())
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
                        a href="/about" { "About" }
                        " | "
                        a href=(blog_url) { "Blog" }
                        " | "
                        a href=(portfolio_url) { "Portfolio" }
                        " | "
                        a href="#" onclick="confirmContact();" { "Contact" }
                    }
                }
                (footer::footer())
            }

            (maud::PreEscaped(r#"
                <script type="module">
                    import init from '/static/frontend/wasm_frontend.js';
                    init();
                </script>
                <script>
                    function confirmContact() {
                        if (confirm("Do you want to send an email?")) {
                            window.location.href = "mailto:one.lemorage@gmail.com?subject=Ti%20Amo&body=Hi%20there,%0D%0A%0D%0A";
                        }
                    }
                </script>
            "#))
        }
    };

    Ok(HttpResponse::Ok()
        .content_type("text/html")
        .body(rendered.into_string()))
}
