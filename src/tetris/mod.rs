pub mod components;
mod systems;

use bevy::prelude::{in_state, App, IntoSystemConfig, Plugin};

use crate::state::GameState;

pub const FALL_COOLDOWN: u64 = 200;

pub struct TetrisPlugin;

impl Plugin for TetrisPlugin {
  fn build(&self, app: &mut App) {
    app
      .init_resource::<resources::FallTimer>()
      .add_event::<events::TetrisMove>()
      .add_event::<events::TetrisPlace>()
      .add_system(systems::fall.run_if(in_state(GameState::Playing)))
      .add_system(systems::move_parts)
      .add_system(systems::snakify)
      .add_system(systems::clear_line)
      .add_system(systems::gravity)
      .add_system(systems::free_fall);
  }
}

pub mod events {
  use bevy::prelude::{Entity, Transform};

  pub enum TetrisMove {
    Down(Entity),
    Left(Entity),
    Right(Entity),
  }

  pub struct TetrisPlace(pub Entity, pub Vec<(Entity, Transform)>);
}

pub mod resources {
  use std::time::Duration;

  use bevy::{
    prelude::Resource,
    time::{Timer, TimerMode},
  };

  use super::FALL_COOLDOWN;

  #[derive(Resource)]
  pub struct FallTimer(pub Timer);

  impl Default for FallTimer {
    fn default() -> Self {
      Self(Timer::new(
        Duration::from_millis(FALL_COOLDOWN),
        TimerMode::Repeating,
      ))
    }
  }

  impl FallTimer {
    pub fn finished(&mut self, delta: Duration) -> bool {
      self.0.tick(delta);
      self.0.finished()
    }
  }
}
