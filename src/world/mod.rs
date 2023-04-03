pub mod styles;
mod systems;

use bevy::prelude::{App, ClearColor, Color, Plugin, Vec2};

pub const BACKGROUND_COLOR: Color = Color::rgb(23. / 255., 23. / 255., 23. / 255.);
pub const CELL_WIDTH: f32 = 20.;
pub const CELL_HEIGHT: f32 = 20.;
pub const CELL_SIZE: Vec2 = Vec2::new(CELL_WIDTH, CELL_HEIGHT);

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
  fn build(&self, app: &mut App) {
    app
      .insert_resource(ClearColor(BACKGROUND_COLOR))
      .add_startup_system(systems::spawn_camera);
  }
}
