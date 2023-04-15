pub mod components;
mod systems;
pub mod utils;

use bevy::{
  prelude::{in_state, App, IntoSystemConfig, Plugin},
  time::common_conditions::on_timer,
};
use std::time::Duration;

use crate::state::GameState;

pub const MAX_SERPENTINE_DURATION: Duration = Duration::from_millis(120);
pub const MIN_SERPENTINE_DURATION: Duration = Duration::from_millis(30);

pub struct SnakePlugin;

impl Plugin for SnakePlugin {
  fn build(&self, app: &mut App) {
    app
      .add_event::<events::SnakeSizeChange>()
      .add_event::<events::Serpentine>()
      .add_system(systems::serpentine.run_if(in_state(GameState::Playing)))
      .add_system(systems::resize)
      .add_system(systems::grow)
      .add_system(systems::eat)
      .add_system(systems::update_score)
      .add_system(systems::seek)
      .add_system(systems::disappear.run_if(on_timer(
        MIN_SERPENTINE_DURATION + (MAX_SERPENTINE_DURATION - MIN_SERPENTINE_DURATION) / 2,
      )))
      .add_system(systems::die);
  }
}

pub mod events {
  use bevy::prelude::{Entity, Vec3};

  pub type SnakeSizeChange = (Entity, BodySizeChange);

  #[derive(Debug)]
  pub enum BodySizeChange {
    Grow,
    Shrink,
  }

  #[derive(Clone, Copy)]
  pub struct Serpentine(pub Entity, pub Vec3);
}
