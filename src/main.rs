mod debug;
mod food;
mod snake;
mod world;

use bevy::{prelude::App, DefaultPlugins};
use debug::DebugPlugin;
use food::FoodPlugin;
use snake::SnakePlugin;
use world::WorldPlugin;

fn main() {
  App::new()
    .add_plugins(DefaultPlugins)
    .add_plugin(WorldPlugin)
    .add_plugin(SnakePlugin)
    .add_plugin(FoodPlugin)
    .add_plugin(DebugPlugin)
    .run();
}
