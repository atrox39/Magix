use wasm_bindgen::prelude::*;
use web_sys::HtmlInputElement;

use crate::{editor::state::EditorState, dom::attach::*};

#[wasm_bindgen]
pub struct ImageEditor {
    state: EditorState,
}

#[wasm_bindgen]
pub struct EditorConfig {
  canvas_id: String,
  file_input_id: Option<String>,
  gray_button_id: Option<String>,
  invert_button_id: Option<String>,
  brightness_slider_id: Option<String>,
  brightness_apply_id: Option<String>,
  undo_button_id: Option<String>,
  redo_button_id: Option<String>,
  export_button_id: Option<String>,
}

#[wasm_bindgen]
impl EditorConfig {
  #[wasm_bindgen(constructor)]
  pub fn new(canvas_id: String) -> EditorConfig {
    EditorConfig {
      canvas_id,
      file_input_id: None,
      gray_button_id: None,
      invert_button_id: None,
      brightness_slider_id: None,
      brightness_apply_id: None,
      undo_button_id: None,
      redo_button_id: None,
      export_button_id: None,
    }
  }

  #[wasm_bindgen(setter)]
  pub fn set_file_input_id(&mut self, id: String) {
    self.file_input_id = Some(id);
  }

  #[wasm_bindgen(setter)]
  pub fn set_gray_button_id(&mut self, id: String) {
    self.gray_button_id = Some(id);
  }

  #[wasm_bindgen(setter)]
  pub fn set_invert_button_id(&mut self, id: String) {
    self.invert_button_id = Some(id);
  }

  #[wasm_bindgen(setter)]
  pub fn set_brightness_slider_id(&mut self, id: String) {
    self.brightness_slider_id = Some(id);
  }

  #[wasm_bindgen(setter)]
  pub fn set_brightness_apply_id(&mut self, id: String) {
    self.brightness_apply_id = Some(id);
  }

  #[wasm_bindgen(setter)]
  pub fn set_undo_button_id(&mut self, id: String) {
    self.undo_button_id = Some(id);
  }

  #[wasm_bindgen(setter)]
  pub fn set_redo_button_id(&mut self, id: String) {
    self.redo_button_id = Some(id);
  }

  #[wasm_bindgen(setter)]
  pub fn set_export_button_id(&mut self, id: String) {
    self.export_button_id = Some(id);
  }
}

#[wasm_bindgen]
impl ImageEditor {

    #[wasm_bindgen(constructor)]
    pub fn new(config: EditorConfig) -> Result<ImageEditor, JsValue> {
        console_error_panic_hook::set_once();

        let document = crate::dom::utils::web_document(
            &crate::dom::utils::web_window()?
        )?;

        let state = EditorState::from_canvas_id(&document, &config.canvas_id)?;

        let editor = ImageEditor { state };

        // Condicionalmente conectar los elementos
        if let Some(id) = config.file_input_id {
            attach_file_input(&document, &editor.state, &id)?;
        }

        if let Some(id) = config.gray_button_id {
            attach_button(&document, &id, {
                let st = editor.state.clone();
                move || st.apply_filter(crate::editor::filters::filter_grayscale)
            })?;
        }

        if let Some(id) = config.invert_button_id {
            attach_button(&document, &id, {
                let st = editor.state.clone();
                move || st.apply_filter(crate::editor::filters::filter_invert)
            })?;
        }

        if let Some(apply_id) = config.brightness_apply_id {
            let slider_id = config.brightness_slider_id
                .ok_or("Brightness slider ID is required")?;

            let slider: HtmlInputElement = document
                .get_element_by_id(&slider_id)
                .ok_or("Brightness slider not found")?
                .dyn_into()?;

            attach_button(&document, &apply_id, {
                let st = editor.state.clone();
                move || {
                    let v = slider.value().parse::<i32>().unwrap_or(0);
                    st.apply_filter(move |px| {
                        crate::editor::filters::filter_brightness(px, v)
                    });
                }
            })?;
        }

        if let Some(id) = config.undo_button_id {
            attach_button(&document, &id, {
                let st = editor.state.clone();
                move || st.undo()
            })?;
        }

        if let Some(id) = config.redo_button_id {
            attach_button(&document, &id, {
                let st = editor.state.clone();
                move || st.redo()
            })?;
        }

        if let Some(id) = config.export_button_id {
            let canvas = editor.state.canvas.clone();
            attach_button(&document, &id, move || {
                if let Ok(url) = canvas.to_data_url() {
                    let _ = web_sys::window().and_then(|w| w.open_with_url(&url).ok());
                }
            })?;
        }

        Ok(editor)
    }

    pub fn leak(self) {
      std::mem::forget(self);
    }
}
