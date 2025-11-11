#[derive(Clone)]
pub struct History {
  past: Vec<Vec<u8>>,
  future: Vec<Vec<u8>>,
  cap: usize,
}

impl History {
  pub fn new(cap: usize) -> Self {
    Self { past: vec![], future: vec![], cap }
  }

  pub fn push(&mut self, buf: Vec<u8>) {
    self.past.push(buf);
    if self.future.len() > self.cap {
      self.future.remove(0);
    }
    self.future.clear();
  }

  pub fn undo(&mut self) -> Option<Vec<u8>> {
    if let Some(last) = self.past.pop() {
      self.future.push(last.clone());
      Some(last)
    } else {
      None
    }
  }

  pub fn redo(&mut self) -> Option<Vec<u8>> {
    if let Some(buf) = self.future.pop() {
      self.past.push(buf.clone());
      Some(buf)
    } else {
      None
    }
  }

  pub fn clear_redo(&mut self) {
    self.future.clear();
  }
}
