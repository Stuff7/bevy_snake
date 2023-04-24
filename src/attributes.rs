pub mod components {
  use std::time::Duration;

  use bevy::prelude::{Bundle, Color, Component};
  use bevy::time::{Timer, TimerMode};

  #[derive(Debug, Component, Default)]
  pub struct Brightness(pub f32);

  #[derive(Debug, Component, Default)]
  pub struct BaseColor(pub Color);

  #[derive(Debug, Component)]
  pub struct MoveCooldown(pub Timer);

  impl MoveCooldown {
    pub fn set_cooldown_duration(&mut self, cooldown: Duration) {
      self.0.set_duration(cooldown);
    }

    pub fn set_cooldown_ms(&mut self, cooldown: u64) {
      self.0.set_duration(Duration::from_millis(if cooldown < 5 {
        5
      } else {
        cooldown
      }));
    }

    pub fn finished(&mut self, delta: Duration) -> bool {
      self.0.tick(delta);
      self.0.finished()
    }
  }

  #[derive(Debug, Component)]
  pub struct Speed(pub f32);

  #[derive(Debug, Bundle)]
  pub struct SpeedBundle {
    move_cooldown: MoveCooldown,
    speed: Speed,
  }

  impl SpeedBundle {
    pub fn new(speed: f32) -> Self {
      Self {
        move_cooldown: MoveCooldown(Timer::new(Duration::from_millis(100), TimerMode::Repeating)),
        speed: Speed(speed),
      }
    }
  }
}

pub mod utils {
  use bevy::prelude::Color;

  pub fn brighten_color(color: &Color, amount: f32) -> Color {
    let brightness = 0.299 * color.r() + 0.587 * color.g() + 0.114 * color.b();
    let new_brightness = brightness + amount;
    let ratio = new_brightness / brightness;
    Color::rgb(color.r() * ratio, color.g() * ratio, color.b() * ratio)
  }

  pub fn desaturate_color(color: &Color, desaturation: f32) -> Color {
    let r = color.r();
    let g = color.g();
    let b = color.b();

    let avg = (r + g + b) / 3.0;

    let r = (r + (avg - r) * desaturation).min(1.0).max(0.0);
    let g = (g + (avg - g) * desaturation).min(1.0).max(0.0);
    let b = (b + (avg - b) * desaturation).min(1.0).max(0.0);

    Color::rgb(r, g, b)
  }
}
