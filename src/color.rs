use bevy::prelude::{App, Plugin};

pub struct ColorPlugin;

impl Plugin for ColorPlugin {
  fn build(&self, app: &mut App) {
    app.add_system(systems::update_brightness);
  }
}

pub mod components {
  use bevy::prelude::{Color, Component};

  #[derive(Debug, Component, Default)]
  pub struct Brightness(pub f32);

  #[derive(Debug, Component, Default)]
  pub struct BaseColor(pub Color);
}

pub mod systems {
  use bevy::prelude::{Changed, Or, Query, Sprite};

  use super::{
    components::{BaseColor, Brightness},
    utils::increase_brightness,
  };

  pub(super) fn update_brightness(
    mut q_sprite: Query<
      (&mut Sprite, &BaseColor, &Brightness),
      Or<(Changed<Brightness>, Changed<BaseColor>)>,
    >,
  ) {
    for (mut sprite, color, brightness) in &mut q_sprite {
      sprite.color = increase_brightness(&color.0, brightness.0);
    }
  }
}

pub mod utils {
  use bevy::prelude::Color;

  pub fn increase_brightness(color: &Color, amount: f32) -> Color {
    let brightness = 0.299 * color.r() + 0.587 * color.g() + 0.114 * color.b();
    let new_brightness = brightness + amount;
    let ratio = new_brightness / brightness;
    Color::rgb(color.r() * ratio, color.g() * ratio, color.b() * ratio)
  }
}
