mod attributes;
mod board;
mod debug;
mod effects;
mod enemy;
mod food;
mod main_camera;
mod player;
mod scoreboard;
mod snake;
mod tetris;

use bevy::{
  prelude::{App, DefaultPlugins, PluginGroup, Window, WindowPlugin},
  window::PresentMode,
};

pub const MAX_MOVE_COOLDOWN: f32 = 200.;

fn main() {
  App::new()
    .add_plugins(DefaultPlugins.set(WindowPlugin {
      primary_window: Some(Window {
        title: "Snake".into(),
        resolution: (800., 800.).into(),
        present_mode: PresentMode::AutoVsync,
        ..Default::default()
      }),
      ..Default::default()
    }))
    .add_state::<state::GameState>()
    .add_plugin(main_camera::MainCameraPlugin)
    .add_plugin(scoreboard::ScoreboardPlugin)
    .add_plugin(effects::EffectPlugin)
    .add_plugin(tetris::TetrisPlugin)
    .add_plugin(board::BoardPlugin)
    .add_plugin(player::PlayerPlugin)
    .add_plugin(enemy::EnemyPlugin)
    .add_plugin(snake::SnakePlugin)
    .add_plugin(food::FoodPlugin)
    .add_plugin(debug::DebugPlugin)
    .run();
}

pub mod state {
  use bevy::prelude::States;

  #[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
  pub enum GameState {
    #[default]
    Paused,
    Playing,
  }
}

mod collections {
  pub trait ExternalOps {
    fn add(&self, rhs: Self) -> Self;
  }

  impl ExternalOps for (f32, f32) {
    fn add(&self, rhs: Self) -> Self {
      (self.0 + rhs.0, self.1 + rhs.1)
    }
  }
}
