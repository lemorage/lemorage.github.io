use maud::{html, Markup};

pub fn footer() -> Markup {
    html! {
        div class="footer-main" {
            "Glenn Miao " (maud::PreEscaped("&copy;")) " 2024. All rights reserved."
        }
    }
}
