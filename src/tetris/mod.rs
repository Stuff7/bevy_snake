pub mod components;
mod systems;

use bevy::prelude::{App, Plugin};

pub struct TetrisPlugin;

impl Plugin for TetrisPlugin {
  fn build(&self, app: &mut App) {
    app.add_system(systems::fall);
  }
}
