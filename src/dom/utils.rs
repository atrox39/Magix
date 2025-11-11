use wasm_bindgen::JsValue;
use web_sys::{Document, Window};

pub fn web_window() -> Result<Window, JsValue> {
  web_sys::window().ok_or("No window".into())
}

pub fn web_document(window: &Window) -> Result<Document, JsValue> {
  window.document().ok_or("No document".into())
}
