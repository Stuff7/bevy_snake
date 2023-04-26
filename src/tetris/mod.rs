pub mod components;
mod systems;

use bevy::prelude::{in_state, App, IntoSystemConfig, Plugin};

use crate::state::GameState;

pub struct TetrisPlugin;

impl Plugin for TetrisPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_event::<events::TetrisMove>()
      .add_system(systems::fall.run_if(in_state(GameState::Playing)))
      .add_system(systems::move_parts)
      .add_system(systems::place)
      .add_system(systems::snakify);
  }
}

pub mod events {
  use bevy::prelude::Entity;

  pub enum TetrisMove {
    Down(Entity),
    Left(Entity),
    Right(Entity),
  }
}
