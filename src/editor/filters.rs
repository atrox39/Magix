pub fn filter_grayscale(px: &mut [u8]) {
  for p in px.chunks_mut(4) {
    let y = (0.2126 * p[0] as f32 + 0.7152 * p[1] as f32 + 0.0722 * p[2] as f32) as u8;
    p[0] = y;
    p[1] = y;
    p[2] = y;
  }
}

pub fn filter_invert(px: &mut [u8]) {
  for p in px.chunks_mut(4) {
    p[0] = 255 - p[0];
    p[1] = 255 - p[1];
    p[2] = 255 - p[2];
  }
}

pub fn filter_brightness(px: &mut [u8], delta: i32) {
  for p in px.chunks_mut(4) {
    p[0] = clamp(p[0] as i32 + delta);
    p[1] = clamp(p[1] as i32 + delta);
    p[2] = clamp(p[2] as i32 + delta);
  }
}

fn clamp(v: i32) -> u8 {
  v.max(0).min(255) as u8
}
