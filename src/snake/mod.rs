pub mod components;
mod systems;
pub mod utils;

use bevy::{
  prelude::{in_state, App, IntoSystemConfig, Plugin},
  time::common_conditions::on_timer,
};
use std::time::Duration;

use crate::state::GameState;

pub const MAX_SERPENTINE_DURATION_MS: u64 = 120;
pub const MIN_SERPENTINE_DURATION_MS: u64 = 30;
pub const SERPENTINE_DURATION_MS: u64 =
  MIN_SERPENTINE_DURATION_MS + (MAX_SERPENTINE_DURATION_MS - MIN_SERPENTINE_DURATION_MS) / 2;

pub const SERPENTINE_DURATION: Duration = Duration::from_millis(SERPENTINE_DURATION_MS);

pub struct SnakePlugin;

impl Plugin for SnakePlugin {
  fn build(&self, app: &mut App) {
    app
      .add_event::<events::SnakeResize>()
      .add_event::<events::Serpentine>()
      .add_system(systems::serpentine.run_if(in_state(GameState::Playing)))
      .add_system(systems::resize)
      .add_system(systems::recolor)
      .add_system(systems::grow)
      .add_system(systems::shrink)
      .add_system(systems::eat)
      .add_system(systems::update_score)
      .add_system(systems::seek)
      .add_system(systems::disappear.run_if(on_timer(SERPENTINE_DURATION)))
      .add_system(systems::die)
      .add_system(systems::revive);
  }
}

pub mod events {
  use bevy::prelude::{Entity, Vec3};

  pub type SnakeResize = (Entity, BodyResize);

  #[derive(Debug)]
  pub enum BodyResize {
    Grow(u32),
    Shrink(u32),
  }

  #[derive(Clone, Copy)]
  pub struct Serpentine(pub Entity, pub Vec3);
}
