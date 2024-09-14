use sycamore::prelude::*;
use wasm_bindgen::prelude::*;
use web_sys::{window, Document, Element};

// Initialize the Sycamore app and inject it into the HTML document
#[wasm_bindgen(start)]
pub fn start() {
    web_sys::console::log_1(&"Wasm and Sycamore app starting...".into());

    let document = window().unwrap().document().unwrap();
    if let Some(root_element) = document.get_element_by_id("sycamore-root") {
        sycamore::render_to(
            |cx| {
                view! { cx,
                    ThemeSwitcher {}
                }
            },
            &root_element,
        );

        web_sys::console::log_1(&"Sycamore component successfully rendered.".into());
    } else {
        web_sys::console::error_1(&"sycamore-root div not found!".into());
    }
}

// Define the ThemeSwitcher component
#[component]
fn ThemeSwitcher<G: Html>(cx: Scope) -> View<G> {
    let is_dark = create_signal(cx, false);

    // Toggle theme logic that modifies body class
    let toggle_theme = move |_| {
        web_sys::console::log_1(&"Theme toggle button clicked.".into());

        let document: Document = window().unwrap().document().unwrap();
        let body: Element = document.body().unwrap().into();

        if *is_dark.get() {
            web_sys::console::log_1(&"Switching to light theme.".into());
            body.set_class_name("light");
        } else {
            web_sys::console::log_1(&"Switching to dark theme.".into());
            body.set_class_name("dark");
        }
        is_dark.set(!*is_dark.get());

        update_icon();
    };

    // Function to update the FontAwesome icon based on the current theme
    fn update_icon() {
        let document = window().unwrap().document().unwrap();
        let theme_icon = document.get_element_by_id("theme-icon").unwrap();

        let is_light_theme = document.body().unwrap().class_list().contains("light");
        if is_light_theme {
            theme_icon.set_class_name("fas fa-lightbulb");
            web_sys::console::log_1(&"Icon updated to lightbulb.".into());
        } else {
            theme_icon.set_class_name("fas fa-moon");
            web_sys::console::log_1(&"Icon updated to moon.".into());
        }
    }

    view! { cx,
        button(class="theme-toggle-btn", on:click=toggle_theme, style="padding: 10px 20px; font-size: 16px; margin-top: 20px; cursor: pointer; position: fixed; top: 20px; right: 20px; z-index: 1000; background: transparent; border: none;") {
            i(id="theme-icon", class="fas fa-lightbulb") {}
        }
    }
}
