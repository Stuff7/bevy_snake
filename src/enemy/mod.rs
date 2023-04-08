mod systems;

use bevy::prelude::{App, Plugin};

pub(super) const INITIAL_ENEMY_LENGTH: usize = 4;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_startup_system(systems::startup)
      .add_system(systems::respawn)
      .add_system(systems::seek_food);
  }
}

pub mod components {
  use bevy::prelude::Component;

  #[derive(Component)]
  pub struct Enemy;
}
