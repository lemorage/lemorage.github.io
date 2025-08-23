use crate::config::Config;
use maud::{html, Markup};

pub fn header(current_page: &str) -> Markup {
    html! {
        @if current_page != "home" {
            header {
                // Live color bar at the top
                div class="color-bar" {}

                div class="nav-container" {
                    nav class="nav-links" {
                        ul {
                            li { a href="/" { "Home" } }
                            @if current_page != "about" {
                                li { a href="/about" { "About" } }
                            }
                            @if current_page != "portfolio" {
                                li { a href=(Config::GITHUB_URL) target="_blank" rel="noopener noreferrer" { "Portfolio" } }
                            }
                            @if current_page != "blog" {
                                li { a href=(Config::BLOG_URL) target="_blank" rel="noopener noreferrer" { "Blog" } }
                            }
                        }
                    }
                }
            }
        }
    }
}
