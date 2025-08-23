use maud::{html, Markup};
use std::time::{SystemTime, UNIX_EPOCH};

fn current_year() -> i32 {
    let secs = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs();

    // Days since epoch, accounting for leap years properly
    let days = secs / 86400;
    let mut year = 1970;
    let mut days_left = days;

    loop {
        let days_in_year = if is_leap_year(year) { 366 } else { 365 };
        if days_left < days_in_year {
            break;
        }
        days_left -= days_in_year;
        year += 1;
    }

    year as i32
}

fn is_leap_year(year: u64) -> bool {
    year % 4 == 0 && (year % 100 != 0 || year % 400 == 0)
}

pub fn footer() -> Markup {
    let year = current_year();
    html! {
        div class="footer-main" {
            "Glenn Miao " (maud::PreEscaped("&copy;")) " " (year) ". All rights reserved."
        }
    }
}
