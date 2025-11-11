use wasm_bindgen::prelude::*;
use web_sys::HtmlImageElement;

use super::state::EditorState;

pub fn load_image_from_data_url(state: EditorState, url: &str) {
  let img = HtmlImageElement::new().unwrap();
  let clone = img.clone();
  let st = state.clone();

  let onload = Closure::<dyn FnMut()>::new(move || {
    st.canvas.set_width(clone.width());
    st.canvas.set_height(clone.height());
    st.ctx
      .draw_image_with_html_image_element(&clone, 0.0, 0.0)
      .unwrap();
    st.push_history();
  });
  img.set_onload(Some(onload.as_ref().unchecked_ref()));
  onload.forget();
  img.set_src(url);
}
