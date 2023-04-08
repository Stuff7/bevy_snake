mod systems;

use bevy::prelude::{App, Plugin};

pub struct FoodPlugin;

impl Plugin for FoodPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_event::<events::FoodEaten>()
      .add_system(systems::spawn)
      .add_system(systems::reposition);
  }
}

pub mod components {
  use bevy::prelude::{Color, Component};
  use rand::{distributions::Standard, prelude::Distribution, Rng};

  #[derive(Debug, Component, Copy, Clone)]
  pub enum Food {
    None,
    Swiftness,
    ExtraGrowth,
  }

  impl Distribution<Food> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Food {
      match rng.gen_range(0..=2) {
        0 => Food::None,
        1 => Food::Swiftness,
        _ => Food::ExtraGrowth,
      }
    }
  }

  impl From<Food> for Color {
    fn from(food: Food) -> Self {
      match food {
        Food::None => Color::rgb(1., 191. / 255., 72. / 255.),
        Food::Swiftness => Color::rgb(135. / 255., 212. / 255., 235. / 255.),
        Food::ExtraGrowth => Color::rgb(143. / 255., 135. / 255., 235. / 255.),
      }
    }
  }
}

pub mod events {
  use bevy::prelude::Entity;

  pub struct FoodEaten {
    pub snake: Entity,
    pub food: Entity,
  }
}
