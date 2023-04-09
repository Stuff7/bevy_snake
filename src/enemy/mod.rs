mod systems;

use bevy::prelude::{on_event, App, IntoSystemConfig, Plugin};

use crate::snake::events::SnakeDeath;

pub(super) const INITIAL_ENEMY_LENGTH: usize = 4;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_event::<events::SpawnEnemy>()
      .add_startup_system(systems::respawn)
      .add_system(systems::spawn)
      .add_system(systems::respawn.run_if(on_event::<SnakeDeath>()))
      .add_system(systems::seek_food);
  }
}

pub mod components {
  use bevy::prelude::Component;

  #[derive(Component)]
  pub struct Enemy;
}

pub mod events {
  pub struct SpawnEnemy;
}
