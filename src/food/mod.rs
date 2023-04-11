mod systems;

use bevy::prelude::{App, Plugin};

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
  use rand::{distributions::Standard, prelude::Distribution, Rng};

  #[derive(Debug, Component, Copy, Clone, PartialEq, Eq)]
  pub enum Food {
    Regular,
    Swiftness,
    ExtraGrowth,
  }

  impl Distribution<Food> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Food {
      match rng.gen_range(0..=2) {
        0 => Food::ExtraGrowth,
        1 => Food::Swiftness,
        _ => Food::ExtraGrowth,
      }
    }
  }

  impl From<Food> for Color {
    fn from(food: Food) -> Self {
      match food {
        Food::Regular => Color::rgb_u8(255, 191, 72),
        Food::Swiftness => Color::rgb_u8(135, 212, 235),
        Food::ExtraGrowth => Color::rgb_u8(143, 135, 235),
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
