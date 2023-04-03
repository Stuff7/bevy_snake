mod systems;

use bevy::prelude::{App, Color, Plugin};

pub const FOOD_COLOR: Color = Color::rgb(1., 191. / 255., 72. / 255.);

pub struct FoodPlugin;

impl Plugin for FoodPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_event::<events::FoodEaten>()
      .add_startup_system(systems::food_spawning)
      .add_system(systems::food_repositioning);
  }
}

pub mod components {
  use bevy::prelude::Component;

  #[derive(Debug, Component)]
  pub struct Food;
}

pub mod events {
  pub struct FoodEaten;
}
