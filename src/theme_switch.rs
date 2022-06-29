use wasm_bindgen::prelude::*;
use web_sys::Element;

pub fn root() -> Element {
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    document
        .document_element()
        .expect("document should have a root element")
}

pub fn set_class(element: web_sys::Element, name: &str) {
    element.set_class_name(&name)
}

#[wasm_bindgen]
pub fn set_theme1() {
    set_class(root(), "theme1");
}

#[wasm_bindgen]
pub fn set_theme2() {
    set_class(root(), "theme2");
}

pub fn hook() {}
