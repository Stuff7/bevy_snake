mod systems;

use bevy::prelude::{App, Color, Plugin};

pub(super) const INITIAL_ENEMY_LENGTH: usize = 4;
pub(super) const OMNIVOROUS_COLOR: Color = Color::rgb(90. / 255., 162. / 255., 250. / 255.);
pub(super) const KILLER_COLOR: Color = Color::rgb(202. / 255., 98. / 255., 157. / 255.);
pub(super) const SPEEDSTER_COLOR: Color = Color::rgb(254. / 255., 227. / 255., 0. / 255.);
pub(super) const GLUTTON_COLOR: Color = Color::rgb(254. / 255., 165. / 255., 1. / 255.);

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_startup_system(systems::spawn_enemies)
      .add_system(systems::respawn)
      .add_system(systems::seek_food)
      .add_system(systems::seek_snake)
      .add_system(systems::seek_speed)
      .add_system(systems::seek_nourishment);
  }
}

pub mod components {
  use bevy::prelude::Component;

  #[derive(Component)]
  pub struct Enemy;

  #[derive(Component)]
  pub struct Omnivorous;

  #[derive(Component)]
  pub struct Killer;

  #[derive(Component)]
  pub struct Speedster;

  #[derive(Component)]
  pub struct Glutton;
}
