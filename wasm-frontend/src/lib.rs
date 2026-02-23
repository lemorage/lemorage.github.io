use sycamore::prelude::*;
use wasm_bindgen::prelude::*;
use web_sys::{window, Document, Element, HtmlElement, MouseEvent};

/// Check if View Transitions API is supported and call startViewTransition
fn try_start_view_transition(document: &Document, callback: &js_sys::Function) -> bool {
    // Check if startViewTransition exists on document
    let start_fn = js_sys::Reflect::get(document, &JsValue::from_str("startViewTransition"))
        .unwrap_or(JsValue::UNDEFINED);

    if start_fn.is_function() {
        // Call document.startViewTransition(callback)
        let func: js_sys::Function = start_fn.unchecked_into();
        let _ = func.call1(document, callback);
        true
    } else {
        false
    }
}

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

    // Toggle theme with ripple effect using View Transitions API
    let toggle_theme = move |event: web_sys::Event| {
        let document: Document = window().unwrap().document().unwrap();
        let body: Element = document.body().unwrap().into();
        let new_is_dark = !*is_dark.get();
        let theme_class = if new_is_dark { "dark" } else { "light" };

        // Get click position from MouseEvent
        let (x, y) = if let Ok(mouse_event) = event.dyn_into::<MouseEvent>() {
            (mouse_event.client_x() as f64, mouse_event.client_y() as f64)
        } else {
            // Fallback to center of viewport
            let win = window().unwrap();
            (
                win.inner_width().unwrap().as_f64().unwrap_or(800.0) / 2.0,
                win.inner_height().unwrap().as_f64().unwrap_or(600.0) / 2.0,
            )
        };

        // Calculate end radius to cover entire viewport
        let win = window().unwrap();
        let vw = win.inner_width().unwrap().as_f64().unwrap_or(800.0);
        let vh = win.inner_height().unwrap().as_f64().unwrap_or(600.0);
        let end_radius = ((x.max(vw - x)).powi(2) + (y.max(vh - y)).powi(2)).sqrt();

        // Set CSS variables for ripple animation
        if let Some(root) = document.document_element() {
            if let Ok(html_el) = root.dyn_into::<HtmlElement>() {
                let style = html_el.style();
                let _ = style.set_property("--ripple-x", &format!("{}px", x));
                let _ = style.set_property("--ripple-y", &format!("{}px", y));
                let _ = style.set_property("--ripple-end-radius", &format!("{}px", end_radius));
            }
        }

        // Try to use View Transitions API, fallback to simple transition
        let theme_class_owned = theme_class.to_string();
        let update_fn = Closure::once(Box::new(move || {
            let doc = window().unwrap().document().unwrap();
            let body: Element = doc.body().unwrap().into();
            body.set_class_name(&theme_class_owned);
            update_icon();
        }) as Box<dyn FnOnce()>);

        if try_start_view_transition(&document, update_fn.as_ref().unchecked_ref()) {
            // View Transitions API supported - animation will play
            update_fn.forget();
        } else {
            // Fallback: apply theme directly with CSS transition
            body.set_class_name(theme_class);
            let root = document.document_element().unwrap();
            let _ = root.class_list().add_1("theme-transitioning");

            // Remove transitioning class after animation
            let cleanup = Closure::once(Box::new(move || {
                let doc = window().unwrap().document().unwrap();
                if let Some(root) = doc.document_element() {
                    let _ = root.class_list().remove_1("theme-transitioning");
                }
                update_icon();
            }) as Box<dyn FnOnce()>);

            let _ = window()
                .unwrap()
                .set_timeout_with_callback_and_timeout_and_arguments_0(
                    cleanup.as_ref().unchecked_ref(),
                    800,
                );
            cleanup.forget();
        }

        save_theme_to_local_storage(new_is_dark);
        is_dark.set(new_is_dark);
    };

    // Function to update the FontAwesome icon based on the current theme
    fn update_icon() {
        let document = window().unwrap().document().unwrap();
        if let Some(theme_icon) = document.get_element_by_id("theme-icon") {
            let is_dark = document.body().unwrap().class_list().contains("dark");
            theme_icon.set_class_name(if is_dark { "fas fa-moon" } else { "fas fa-sun" });
        }
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
    let is_about_page = body
        .get_attribute("data-page")
        .map_or(false, |val| val == "about");
    let theme_key = if is_about_page {
        "about-theme"
    } else {
        "theme"
    };

    // Check page-specific local storage
    if let Some(storage) = window().unwrap().local_storage().unwrap() {
        if let Ok(Some(theme)) = storage.get_item(theme_key) {
            body.set_class_name(&theme);
            return theme == "dark";
        }
    }

    // Default: dark for About page, light for others
    let (theme_class, is_dark) = if is_about_page {
        ("dark", true)
    } else {
        ("light", false)
    };
    body.set_class_name(theme_class);
    is_dark
}

// Helper function to save the theme in local storage
fn save_theme_to_local_storage(is_dark: bool) {
    if let Some(storage) = window().unwrap().local_storage().unwrap() {
        let document = window().unwrap().document().unwrap();
        let body = document.body().unwrap();
        let is_about_page = body
            .get_attribute("data-page")
            .map_or(false, |val| val == "about");
        let theme_key = if is_about_page {
            "about-theme"
        } else {
            "theme"
        };
        let theme = if is_dark { "dark" } else { "light" };
        storage.set_item(theme_key, theme).unwrap();
    }
}
