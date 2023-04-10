mod systems;
pub mod utils;

use bevy::prelude::{App, Color, Plugin, Vec2};

pub const BOARD_COLOR: Color = Color::rgb(23. / 255., 23. / 255., 23. / 255.);
pub const CELL_SIZE: f32 = 20.;
pub const HALF_CELL_SIZE: f32 = CELL_SIZE / 2.;
pub const BOARD_WIDTH_FACTOR: f32 = 0.7 / (CELL_SIZE * 2.);
pub const BOARD_HEIGHT_FACTOR: f32 = 0.9 / (CELL_SIZE * 2.);
pub const CELL_SIZE_VEC: Vec2 = Vec2::splat(CELL_SIZE - 4.);

pub struct BoardPlugin;

impl Plugin for BoardPlugin {
  fn build(&self, app: &mut App) {
    app
      .init_resource::<resources::GameBoard>()
      .add_startup_system(systems::spawn)
      .add_system(systems::resize_game_board)
      .add_system(systems::constraint_children);
  }
}

pub mod components {
  use bevy::prelude::Component;

  #[derive(Debug, Component)]
  pub struct Board;

  #[derive(Debug, Component)]
  pub struct BoardSprite;
}

pub mod resources {
  use bevy::prelude::Resource;

  #[derive(Debug, Resource, Default)]
  pub struct GameBoard {
    pub width: f32,
    pub height: f32,
  }
}
