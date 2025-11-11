use wasm_bindgen::prelude::*;
use web_sys::{
  Document, Event, FileReader, HtmlButtonElement, HtmlInputElement,
};

use crate::editor::{state::EditorState, loader::load_image_from_data_url};

pub fn attach_button<F: 'static + FnMut()>(
  document: &Document,
  id: &str,
  mut handler: F,
) -> Result<(), JsValue> {
  let btn: HtmlButtonElement = document
    .get_element_by_id(id)
    .ok_or(format!("Button '{}' not found", id))?
    .dyn_into()?;

  let closure = Closure::<dyn FnMut(Event)>::new(move |_e| handler());
  btn.add_event_listener_with_callback("click", closure.as_ref().unchecked_ref())?;
  closure.forget();

  Ok(())
}

pub fn attach_file_input(
  document: &Document,
  state: &EditorState,
  id: &str,
) -> Result<(), JsValue> {
  let input: HtmlInputElement = document
    .get_element_by_id(id)
    .ok_or("File input not found")?
    .dyn_into()?;

  let input_clone = input.clone();
  let st = state.clone();

  web_sys::console::log_1(&format!("input_clone existe? {:?}", input_clone.is_null()).into());

  let closure = Closure::<dyn FnMut(Event)>::new(move |_evt| {
    if let Some(files) = input_clone.files() {
      if let Some(file) = files.get(0) {
        let fr = FileReader::new().unwrap();
        let fr_clone = fr.clone();
        let st2 = st.clone();
        let onload = Closure::<dyn FnMut(Event)>::new(move |_e| {
          if let Ok(result) = fr_clone.result() {
            if let Some(url) = result.as_string() {

              load_image_from_data_url(st2.clone(), &url);
            }
          }
        });

        fr.set_onload(Some(onload.as_ref().unchecked_ref()));
        onload.forget();

        fr.read_as_data_url(&file).unwrap();
      }
    }
  });

  input.add_event_listener_with_callback("change", closure.as_ref().unchecked_ref())?;
  closure.forget();

  Ok(())
}
