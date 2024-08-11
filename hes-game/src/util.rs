use base64::prelude::*;
use extend::ext;
use hes_engine::flavor::{Image, ImageData};
use html::ElementDescriptor;
use leptos::{wasm_bindgen::JsCast, *};
use leptos_use::use_window;
use web_sys::HtmlCollection;

/// Iteratively scale text (by decreasing the font size) until it fits
/// or reaches the `min_size`.
pub fn scale_text(elem: web_sys::HtmlElement, min_size: u32) {
    if let Some(initial_font_size) = get_font_size(&elem) {
        let mut font_size = initial_font_size as u32;
        while (elem.scroll_height() > elem.client_height()
            || elem.scroll_width() > elem.client_width())
            && font_size > min_size
        {
            let next_size = font_size - 1;
            let _ = elem.style().set_property(
                "font-size",
                &format!("{next_size}px"),
            );
            font_size = next_size;
        }
    }
}

/// Get the font size of a given element.
fn get_font_size(elem: &web_sys::HtmlElement) -> Option<i32> {
    window().get_computed_style(elem).unwrap().map(|style| {
        style
            .get_property_value("font-size")
            .unwrap()
            .replace("px", "")
            .parse::<f32>()
            .unwrap_or(15.)
            .round() as i32
    })
}

/// Guess if this browser is a Safari browser.
pub fn is_safari() -> bool {
    let window = use_window();
    window
        .navigator()
        .map(|navigator| {
            if let Ok(agent) = navigator.user_agent() {
                agent.contains("Safari")
                    && !agent.contains("Chrome")
            } else {
                false
            }
        })
        .unwrap_or_default()
}

/// Guess if we're running on Steam.
pub fn is_steam() -> bool {
    std::option_env!("PLATFORM") != Some("STEAM")
}

/// Identify the index of the child in the center
/// of this element.
pub fn detect_center_element(
    parent: web_sys::HtmlElement,
    elements: &[web_sys::HtmlElement],
) -> Option<usize> {
    let rect = parent.get_bounding_client_rect();
    let target_x = rect.x() + parent.client_width() as f64 / 2.;
    let mut min_dist = f64::INFINITY;
    let mut closest = None;

    for (idx, element) in elements.iter().enumerate() {
        let rect = element.get_bounding_client_rect();
        let pos = rect.x() + rect.width() / 2.;
        let dist = (target_x - pos).abs();
        if dist < min_dist {
            min_dist = dist;
            closest = Some(idx);
        }
    }
    closest
}

/// Convert a `NodeList` to a vec of elements.
pub fn nodelist_to_elements(
    nodelist: web_sys::NodeList,
) -> Vec<web_sys::HtmlElement> {
    (0..nodelist.length())
        .filter_map(|i| nodelist.item(i))
        .filter_map(|node| {
            node.dyn_into::<web_sys::HtmlElement>().ok()
        })
        .collect()
}

/// Convert an `HtmlCollection` to a vec of elements.
pub fn collection_to_elements(
    collection: HtmlCollection,
) -> Vec<web_sys::HtmlElement> {
    let mut elements = Vec::new();
    for i in 0..collection.length() {
        if let Some(element) = collection.item(i) {
            if let Ok(html_element) =
                element.dyn_into::<web_sys::HtmlElement>()
            {
                elements.push(html_element);
            }
        }
    }
    elements
}

/// Convert from a `leptos::HtmlElement` to a `web_sys::HtmlElement`.
pub fn to_ws_el<T: ElementDescriptor + 'static>(
    el: HtmlElement<T>,
) -> web_sys::HtmlElement {
    let el = el.into_any();
    let el: &web_sys::HtmlElement = el.as_ref();
    el.clone()
}

/// Adjust card scale depending on the screen height.
pub fn card_scale() -> f32 {
    let height = use_window()
        .document()
        .body()
        .expect("Will have a body client-side")
        .client_height();
    if height < 600 {
        0.9
    } else {
        1.0
    }
}

#[ext]
pub impl Image {
    fn src(&self) -> String {
        match &self.data {
            ImageData::File(fname) => {
                format!("/assets/content/images/{fname}",)
            }
            ImageData::Data { bytes, mime } => format!(
                "data:{mime};charset=utf-8;base64,{}",
                BASE64_STANDARD.encode(bytes)
            ),
        }
    }
}
