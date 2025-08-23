use sycamore::prelude::*;
use wasm_bindgen::prelude::*;
use web_sys::{window, Document, Element};

// Initialize the Sycamore app and inject it into the HTML document
#[cfg_attr(not(target_arch = "wasm32"), allow(unexpected_cfgs))]
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
    let is_dark = create_signal(cx, load_and_apply_theme());

    // Toggle theme logic that modifies body class
    let toggle_theme = move |_| {
        let document: Document = window().unwrap().document().unwrap();
        let body: Element = document.body().unwrap().into();

        let new_is_dark = !*is_dark.get();
        let theme_class = if new_is_dark { "dark" } else { "light" };
        
        body.set_class_name(theme_class);
        save_theme_to_local_storage(new_is_dark);
        is_dark.set(new_is_dark);
        update_icon();
    };

    // Function to update the FontAwesome icon based on the current theme
    fn update_icon() {
        let document = window().unwrap().document().unwrap();
        let theme_icon = document.get_element_by_id("theme-icon").unwrap();
        let is_dark = document.body().unwrap().class_list().contains("dark");
        
        theme_icon.set_class_name(if is_dark { "fas fa-moon" } else { "fas fa-sun" });
    }

    view! { cx,
        button(class="theme-toggle-btn", on:click=toggle_theme, style="padding: 10px 20px; font-size: 16px; margin-top: 20px; cursor: pointer; position: fixed; top: 18px; left: 16px; z-index: 1000; background: transparent; border: none;") {
            i(id="theme-icon", class=(if *is_dark.get() { "fas fa-moon" } else { "fas fa-sun" })) {}
        }
    }
}

// Helper function to load the theme from local storage and apply it
pub fn load_and_apply_theme() -> bool {
    let document = window().unwrap().document().unwrap();
    let body = document.body().unwrap();
    let is_about_page = body.get_attribute("data-page").map_or(false, |val| val == "about");
    let theme_key = if is_about_page { "about-theme" } else { "theme" };

    // Check page-specific local storage
    if let Some(storage) = window().unwrap().local_storage().unwrap() {
        if let Ok(Some(theme)) = storage.get_item(theme_key) {
            body.set_class_name(&theme);
            return theme == "dark";
        }
    }

    // Default: dark for About page, light for others
    let (theme_class, is_dark) = if is_about_page { ("dark", true) } else { ("light", false) };
    body.set_class_name(theme_class);
    is_dark
}

// Helper function to save the theme in local storage
fn save_theme_to_local_storage(is_dark: bool) {
    if let Some(storage) = window().unwrap().local_storage().unwrap() {
        let document = window().unwrap().document().unwrap();
        let body = document.body().unwrap();
        let is_about_page = body.get_attribute("data-page").map_or(false, |val| val == "about");
        let theme_key = if is_about_page { "about-theme" } else { "theme" };
        let theme = if is_dark { "dark" } else { "light" };
        storage.set_item(theme_key, theme).unwrap();
    }
}
