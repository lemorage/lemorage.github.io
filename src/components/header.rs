use maud::{html, Markup};

pub fn header(current_page: &str) -> Markup {
    html! {
        @if current_page != "home" {
            header {
                nav {
                    a href="/" { "Home" }
                    @if current_page != "about" {
                        a href="/about" { "About Me" }
                    }
                    @if current_page != "portfolio" {
                        a href="/portfolio" { "Portfolio" }
                    }
                    @if current_page != "blog" {
                        a href="/blog" { "Blog" }
                    }
                }
            }
        }
    }
}
