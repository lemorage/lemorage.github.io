use crate::components;
use actix_web::{HttpResponse, Result};
use components::{footer, header, meta_tags};
use maud::{html, Markup, PreEscaped, DOCTYPE};
use pulldown_cmark::{html as md_to_html, Options, Parser};

pub async fn about() -> Result<HttpResponse> {
    let bio_md = r#"
Hi there! I’m Glenn Miao. Acknowledging the fact that, to fully describe a human is a bit of a complex task, I'll mention my experiences and interests specifically below.

I studied archaeology in my undergraduate years and worked as a field crew member before.

Over time, my interest in software development grew, and I transitioned into a formal developer role. I worked as a Database TA and also worked at a startup in Japan, focusing on LLM (RAG) Agents and Web Development before.

I work a lot in **TypeScript** and **Python**, and write **Rust** as well outside of my work.

I’m deeply interested in functional programming and type systems. I have studied several languages in that paradigm, including Scheme, SML, and others (Haskell and OCaml are still on the list!). Besides, I’m fascinated by databases—especially in areas like OLAP, lakehouse architectures, and stream processing.

Outside of tech, I’m passionate about music, especially jazz lately. I’ve dabbled in a number of instruments over the years—drums, keyboards, guitar, trumpet, and more.

I believe in the concept of **equal contribution** and **open source**, and I spend most of my time these days engaging with **OSS** community.
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
            body class="light" data-page="about" {
                (header::header("about"))
                div id="sycamore-root" {}

                main class="cyber-grid" {
                    section class="intro-pod" {
                        div class="avatar-pod" {
                            div class="pixel-avatar" {
                                img src="./static/pixel-me.png" alt="Glenn Miao" class="back";
                            }
                            h1 class="neon-title" data-text="Glenn Miao" { "Glenn Miao" }
                        }
                    }
                    section class="bio-pod" {
                        div class="bio-core pixel-card" {
                            (PreEscaped(bio_html))
                        }
                    }
                    section class="link-pod" {
                        ul class="cyber-links" {
                            li { a href="https://github.com/lemorage" target="_blank" { i class="fab fa-github cyber-icon" {} } }
                            li { a href="https://x.com/LemorageOne" target="_blank" { i class="fab fa-x-twitter cyber-icon" {} } }
                        }
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
