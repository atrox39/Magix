use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::{Clamped, JsCast, JsValue};
use web_sys::{HtmlCanvasElement, CanvasRenderingContext2d, Document, ImageData};

use super::{history::History};

#[derive(Clone)]
pub struct EditorState {
  pub canvas: HtmlCanvasElement,
  pub ctx: CanvasRenderingContext2d,
  pub history: Rc<RefCell<History>>,
}

impl EditorState {
  pub fn from_canvas_id(document: &Document, id: &str) -> Result<Self, JsValue> {
    let canvas: HtmlCanvasElement = document
      .get_element_by_id(id)
      .ok_or("Canvas not found")?
      .dyn_into()?;
    let ctx = canvas
      .get_context("2d")?
      .ok_or("Could not get 2D context")?
      .dyn_into()?;
    Ok(Self::new(canvas, ctx))
  }

  pub fn new(canvas: HtmlCanvasElement, ctx: CanvasRenderingContext2d) -> Self {
    Self { canvas, ctx, history: Rc::new(RefCell::new(History::new(20))) }
  }

  pub fn get_image_data(&self) -> ImageData {
    self
      .ctx
      .get_image_data(0.0, 0.0, self.canvas.width().into(), self.canvas.height().into())
      .unwrap()
  }

  pub fn put_image_data(&self, data: &ImageData) {
    let _ = self.ctx.put_image_data(data, 0.0, 0.0);
  }

  pub fn push_history(&self) {
    let data = self.get_image_data().data().0.clone();
    self.history.borrow_mut().push(data);
  }

  pub fn apply_filter<F>(&self, filter: F)
  where F: Fn(&mut [u8])
  {
    self.push_history();
    let id = self.get_image_data();
    let mut buf = id.data().0.clone();

    filter(&mut buf);
    
    let new = ImageData::new_with_u8_clamped_array_and_sh(
      Clamped(&buf[..]),
      id.width(),
      id.height(),
    )
    .unwrap();
    self.put_image_data(&new);
    self.history.borrow_mut().clear_redo();
  }

  pub fn undo(&self) {
    if let Some(buf) = self.history.borrow_mut().undo() {
      self.draw_bytes(&buf);
    }
  }

  pub fn redo(&self) {
    if let Some(buf) = self.history.borrow_mut().redo() {
      self.draw_bytes(&buf);
    }
  }

  pub fn draw_bytes(&self, buf: &[u8]) {
    let img = ImageData::new_with_u8_clamped_array_and_sh(
      Clamped(buf),
      self.canvas.width(),
      self.canvas.height(),
    )
    .unwrap();
    self.put_image_data(&img);
  }
}
