use bevy::prelude::Color;
use rand::{thread_rng, Rng};

pub fn generate_bright_color() -> Color {
  let mut rng = thread_rng();
  let min_value = 0.3;
  let threshold = 0.5;

  let r: f32 = rng.gen_range(0.0..1.0);
  let g: f32 = rng.gen_range(0.0..1.0);
  let b: f32 = rng.gen_range(0.0..1.0);

  let brightness = 0.299 * r + 0.587 * g + 0.114 * b;

  if brightness < threshold {
    Color::rgb(r.max(min_value), g.max(min_value), b.max(min_value))
  } else {
    Color::rgb(r, g, b)
  }
}
