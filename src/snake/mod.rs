pub mod components;
mod systems;
pub mod utils;

use bevy::{
  prelude::{App, IntoSystemConfig, Plugin},
  time::common_conditions::on_timer,
};
use std::time::Duration;

pub struct SnakePlugin;

impl Plugin for SnakePlugin {
  fn build(&self, app: &mut App) {
    app
      .add_event::<events::SnakeSizeChange>()
      .add_event::<events::Serpentine>()
      .add_event::<events::SnakeDeath>()
      .add_system(systems::serpentine)
      .add_system(systems::resize)
      .add_system(systems::eat)
      .add_system(systems::despawn.run_if(on_timer(Duration::from_secs_f32(0.1))))
      .add_system(systems::die);
  }
}

pub mod events {
  use bevy::prelude::{Entity, Vec3};

  pub type SnakeSizeChange = (Entity, BodySizeChange);

  #[derive(Debug)]
  pub enum BodySizeChange {
    Grow(usize),
    Shrink(usize),
  }

  #[derive(Clone, Copy)]
  pub struct Serpentine(pub Entity, pub Vec3);

  pub struct SnakeDeath;
}
