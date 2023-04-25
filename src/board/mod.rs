pub mod components;
mod systems;

use bevy::prelude::{App, Color, IntoSystemConfig, Plugin, StartupSet, Vec2};

pub const BOARD_COLOR: Color = Color::rgb(23. / 255., 23. / 255., 23. / 255.);
pub const CELL_SIZE: f32 = 16.;
pub const HALF_CELL_SIZE: f32 = CELL_SIZE / 2.;
pub const BOARD_WIDTH_FACTOR: f32 = 0.7 / (CELL_SIZE * 2.);
pub const BOARD_HEIGHT_FACTOR: f32 = 0.9 / (CELL_SIZE * 2.);
pub const CELL_SIZE_VEC: Vec2 = Vec2::splat(CELL_SIZE - 4.);

pub struct BoardPlugin;

impl Plugin for BoardPlugin {
  fn build(&self, app: &mut App) {
    app
      .init_resource::<resources::GameBoard>()
      .add_startup_system(systems::spawn.in_base_set(StartupSet::PreStartup))
      .add_system(systems::resize_game_board)
      .add_system(systems::add_cell_to_board)
      .add_system(systems::constraint_children);
  }
}

pub mod resources {
  use super::{BOARD_HEIGHT_FACTOR, BOARD_WIDTH_FACTOR, CELL_SIZE};
  use bevy::prelude::Resource;

  #[derive(Debug, Resource, Default)]
  pub struct GameBoard {
    pub width: f32,
    pub height: f32,
  }

  impl GameBoard {
    pub fn resize(&mut self, width: f32, height: f32) {
      self.width = 2. * CELL_SIZE * (width * BOARD_WIDTH_FACTOR).floor();
      self.height = 2. * CELL_SIZE * (height * BOARD_HEIGHT_FACTOR).floor();
    }
  }
}

pub mod utils {
  use super::{CELL_SIZE, HALF_CELL_SIZE};
  use bevy::prelude::Vec3;

  pub fn get_board_position(x: f32, y: f32) -> Vec3 {
    Vec3::new(
      (x / CELL_SIZE).floor() * CELL_SIZE + HALF_CELL_SIZE,
      (y / CELL_SIZE).floor() * CELL_SIZE + HALF_CELL_SIZE,
      0.,
    )
  }
}
