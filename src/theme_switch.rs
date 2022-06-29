use wasm_bindgen::prelude::*;
use web_sys::Element;

#[wasm_bindgen]
pub fn set_theme1() {
    set_class(root(), "theme1");
}

#[wasm_bindgen]
pub fn set_theme2() {
    set_class(root(), "theme2");
}

pub fn root() -> Element {
    let window = web_sys::window().expect_throw("no global `window` exists");
    let document = window
        .document()
        .expect_throw("should have a document on window");
    document
        .document_element()
        .expect_throw("document should have a root element")
}

pub fn set_class(element: web_sys::Element, name: &str) {
    element.set_class_name(&name)
}
