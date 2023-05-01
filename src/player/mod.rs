mod systems;

use bevy::prelude::{App, Color, Plugin};

pub(super) const PLAYER_COLOR: Color = Color::rgb(115. / 255., 170. / 255., 115. / 255.);
pub(super) const INITIAL_PLAYER_LENGTH: usize = 4;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_event::<events::RespawnPlayer>()
      .add_startup_system(systems::spawn)
      .add_system(systems::respawn)
      .add_system(systems::queue_snake_input)
      .add_system(systems::iter_snake_input)
      .add_system(systems::tetris_input);
  }
}

pub mod components {
  use crate::snake::components::Direction;
  use bevy::prelude::Component;

  #[derive(Component)]
  pub struct Player;

  #[derive(Debug, Component, Default)]
  pub struct DirectionQueue {
    pub(super) previous: Direction,
    pub(super) next: Option<Direction>,
  }
}

pub mod events {
  pub struct RespawnPlayer;
}
