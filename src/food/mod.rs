mod systems;

use bevy::prelude::{App, Color, Plugin};

pub const FOOD_COLOR: Color = Color::rgb(1., 191. / 255., 72. / 255.);

pub struct FoodPlugin;

impl Plugin for FoodPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_event::<events::FoodEaten>()
      .add_startup_system(systems::spawn)
      .add_system(systems::reposition);
  }
}

pub mod components {
  use bevy::prelude::Component;

  #[derive(Debug, Component)]
  pub struct Food;
}

pub mod events {
  use bevy::prelude::Entity;

  pub struct FoodEaten(pub Entity);
}
