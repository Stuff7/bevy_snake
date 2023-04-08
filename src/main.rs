mod board;
pub mod color;
mod debug;
mod enemy;
mod food;
mod main_camera;
mod player;
mod snake;

use bevy::{
  prelude::{App, DefaultPlugins, PluginGroup, Window, WindowPlugin},
  window::PresentMode,
};

fn main() {
  App::new()
    .add_plugins(DefaultPlugins.set(WindowPlugin {
      primary_window: Some(Window {
        title: "Snake".into(),
        resolution: (board::BOARD_SIZE, board::BOARD_SIZE).into(),
        present_mode: PresentMode::AutoVsync,
        ..Default::default()
      }),
      ..Default::default()
    }))
    .add_plugin(main_camera::MainCameraPlugin)
    .add_plugin(board::BoardPlugin)
    .add_plugin(player::PlayerPlugin)
    .add_plugin(enemy::EnemyPlugin)
    .add_plugin(snake::SnakePlugin)
    .add_plugin(food::FoodPlugin)
    .add_plugin(debug::DebugPlugin)
    .run();
}

mod collections {
  pub trait TupleOps {
    fn add(&self, rhs: Self) -> Self;
  }

  impl TupleOps for (f32, f32) {
    fn add(&self, rhs: Self) -> Self {
      (self.0 + rhs.0, self.1 + rhs.1)
    }
  }
}
