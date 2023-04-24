mod systems;

use bevy::prelude::{App, Color, Plugin};

pub const COLOR_REGULAR: Color = Color::rgb(1.4, 0.8, 2.);
pub const COLOR_BEEFY: Color = Color::rgb(2., 0.5, 2.);
pub const COLOR_ENERGETIC: Color = Color::rgb(2., 1.6, 0.);
pub const COLOR_FREEZE: Color = Color::rgb(0.7, 1.2, 2.);

pub struct FoodPlugin;

impl Plugin for FoodPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_event::<events::SpawnFood>()
      .add_event::<events::FoodEaten>()
      .add_startup_system(systems::startup)
      .add_system(systems::spawn)
      .add_system(systems::reposition)
      .add_system(systems::apply_effects);
  }
}

pub mod components {
  use bevy::prelude::{Color, Component};

  use super::{COLOR_BEEFY, COLOR_ENERGETIC, COLOR_FREEZE, COLOR_REGULAR};

  #[derive(Debug, Component, Copy, Clone, PartialEq, Eq)]
  pub enum Food {
    Regular,
    Energetic,
    Beefy,
    Frozen,
  }

  impl From<Food> for Color {
    fn from(food: Food) -> Self {
      match food {
        Food::Regular => COLOR_REGULAR,
        Food::Energetic => COLOR_ENERGETIC,
        Food::Beefy => COLOR_BEEFY,
        Food::Frozen => COLOR_FREEZE,
      }
    }
  }
}

pub mod events {
  use super::components::Food;
  use bevy::prelude::Entity;

  pub struct FoodEaten {
    pub snake: Entity,
    pub food: Entity,
  }

  pub struct SpawnFood(pub Food);
}
