mod systems;
pub mod utils;

use bevy::prelude::{App, ClearColor, Color, Plugin, Vec2};

pub const BACKGROUND_COLOR: Color = Color::rgb(8. / 255., 8. / 255., 8. / 255.);
pub const BOARD_COLOR: Color = Color::rgb(23. / 255., 23. / 255., 23. / 255.);
pub const CELL_SIZE: f32 = 20.;
pub const HALF_CELL_SIZE: f32 = CELL_SIZE / 2.;
pub const BOARD_CELL_COUNT: f32 = 40.;
pub const BOARD_SIZE: f32 = CELL_SIZE * BOARD_CELL_COUNT;
pub const CELL_SIZE_VEC: Vec2 = Vec2::splat(CELL_SIZE - 4.);

pub struct BoardPlugin;

impl Plugin for BoardPlugin {
  fn build(&self, app: &mut App) {
    app
      .insert_resource(ClearColor(BACKGROUND_COLOR))
      .add_startup_system(systems::spawn_camera);
  }
}

pub mod components {
  use bevy::prelude::Component;

  #[derive(Debug, Component)]
  pub struct Board;

  #[derive(Debug, Component)]
  pub struct MainCamera;
}
