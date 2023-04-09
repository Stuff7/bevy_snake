mod styles;
mod systems;

use bevy::prelude::{App, Plugin};

use self::events::ScoreUpdate;

pub struct ScoreboardPlugin;

impl Plugin for ScoreboardPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_event::<ScoreUpdate>()
      .add_startup_system(systems::spawn)
      .add_system(systems::update_scoreboard)
      .add_system(systems::update_score);
  }
}

pub mod components {
  use bevy::prelude::Component;

  #[derive(Component)]
  pub struct Scoreboard;

  #[derive(Debug, Component)]
  pub struct Score(pub usize);

  #[derive(Debug, Component)]
  pub struct ScoreContainer;

  #[derive(Debug, Component)]
  pub struct ScoreValue;

  #[derive(Debug, Component)]
  pub struct Name(pub String);
}

pub mod events {
  pub struct ScoreUpdate;
}
