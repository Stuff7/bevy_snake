mod debug;
mod food;
mod player;
mod snake;
mod world;

use bevy::{prelude::App, DefaultPlugins};

fn main() {
  App::new()
    .add_plugins(DefaultPlugins)
    .add_plugin(world::WorldPlugin)
    .add_plugin(player::PlayerPlugin)
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
