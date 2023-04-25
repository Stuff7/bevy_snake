pub mod components;
mod systems;

use bevy::prelude::{in_state, App, IntoSystemConfig, Plugin};

use crate::state::GameState;

pub struct TetrisPlugin;

impl Plugin for TetrisPlugin {
  fn build(&self, app: &mut App) {
    app.add_system(systems::fall.run_if(in_state(GameState::Playing)));
  }
}
