use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn set_theme1() {
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let element = document
        .document_element()
        .expect("document should have a root element");
    element.set_class_name("theme1");
}

#[wasm_bindgen]
pub fn set_theme2() {
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let element = document
        .document_element()
        .expect("document should have a root element");
    element.set_class_name("theme2");
}
