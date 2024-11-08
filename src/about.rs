use crate::components;
use actix_web::{HttpResponse, Result};
use components::{footer, header, meta_tags};
use maud::{html, Markup, PreEscaped, DOCTYPE};
use pulldown_cmark::{html as md_to_html, Options, Parser};

pub async fn about() -> Result<HttpResponse> {
    let bio_md = r#"
Glenn Miao is a developer specializing in **LLM Agents** and **RAG pipelines**, with extensive experience in Web Development, and AI integration. Currently, Glenn works for an startup -- [Wondervoy](https://wondervoy.com), focusing on **LLM agents** and leveraging prior experiences in **database systems** to build scalable RAG pipelines for business applications.

While Glenn primarily codes in **TypeScript** and **Python**, he has a particular fondness for **functional programming paradigms** and a strong interest in **Rust**.

Glenn is a staunch advocate of **equal contribution** and **open source**, principles that guide his life. He firmly believes in the potential of open-source software and open research.
    "#;

    // Parse the Markdown content and render to HTML
    let options = Options::empty();
    let parser = Parser::new_ext(bio_md, options);

    let mut bio_html = String::new();
    md_to_html::push_html(&mut bio_html, parser);

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
                    div class="bio-container" {
                        (PreEscaped(bio_html))
                    }
                }
                (footer::footer())
            }
            (maud::PreEscaped(r#"
                <script type="module">
                    import init from '/static/frontend/wasm_frontend.js';
                    init();
                </script>
            "#))
        }
    };

    Ok(HttpResponse::Ok()
        .content_type("text/html")
        .body(rendered.into_string()))
}
