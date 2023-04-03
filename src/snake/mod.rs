pub mod components;
mod systems;

use bevy::{
  prelude::{App, Color, IntoSystemConfig, Plugin},
  time::common_conditions::on_timer,
};
use std::time::Duration;

pub(super) const PLAYER_COLOR: Color = Color::rgb(115. / 255., 170. / 255., 115. / 255.);
pub(super) const INITIAL_TAIL_LENGTH: usize = 4;

pub struct SnakePlugin;

impl Plugin for SnakePlugin {
  fn build(&self, app: &mut App) {
    app
      .add_event::<events::BodySizeChange>()
      .add_event::<events::Serpentine>()
      .add_startup_system(systems::snake_spawning)
      .add_system(systems::snake_steering)
      .add_system(
        systems::snake_head_positioning.run_if(on_timer(Duration::from_secs_f32(0.1))),
      )
      .add_system(systems::snake_serpentining)
      .add_system(systems::snake_resizing)
      .add_system(systems::snake_eating)
      .add_system(systems::snake_dying);
  }
}

pub mod events {
  use bevy::prelude::Vec3;

  pub enum BodySizeChange {
    Grow(usize),
    Shrink(usize),
  }

  #[derive(Clone, Copy)]
  pub struct Serpentine(pub(super) Vec3);
}
