mod systems;

use bevy::prelude::{App, Plugin};

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_system(systems::god_mode)
      .add_system(systems::print_debug_info);
  }
}
