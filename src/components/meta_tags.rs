use maud::{html, Markup};

pub fn meta_tags() -> Markup {
    html! {
        link rel="apple-touch-icon" sizes="180x180" href="/public_assets/apple-touch-icon.png";
        link rel="icon" type="image/png" sizes="32x32" href="/public_assets/favicon-32x32.png";
        link rel="icon" type="image/png" sizes="16x16" href="/public_assets/favicon-16x16.png";
        link rel="icon" type="image/png" sizes="192x192" href="/public_assets/android-chrome-192x192.png";
        link rel="icon" type="image/png" sizes="512x512" href="/public_assets/android-chrome-512x512.png";
        link rel="shortcut icon" href="/public_assets/favicon.ico";
        link rel="stylesheet" href="/static/style.css";
        link rel="stylesheet" href="https://cdnjs.cloudflare.com/ajax/libs/font-awesome/6.0.0-beta3/css/all.min.css";
    }
}
