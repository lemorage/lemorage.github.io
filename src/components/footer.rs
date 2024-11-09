use maud::{html, Markup};
use std::time::{SystemTime, UNIX_EPOCH};

fn current_year() -> i32 {
    let secs_since_epoch = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs();

    1970 + (secs_since_epoch / 31_557_600) as i32
}

pub fn footer() -> Markup {
    let year = current_year();
    html! {
        div class="footer-main" {
            "Glenn Miao " (maud::PreEscaped("&copy;")) " " (year) ". All rights reserved."
        }
    }
}
