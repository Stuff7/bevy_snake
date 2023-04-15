mod styles;
mod systems;

use bevy::prelude::{App, Plugin};

use self::events::ScoreUpdate;

pub struct ScoreboardPlugin;

impl Plugin for ScoreboardPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_event::<ScoreUpdate>()
      .add_startup_system(systems::spawn_scoreboard)
      .add_system(systems::add_score)
      .add_system(systems::sort_scores)
      .add_system(systems::position_scores)
      .add_system(systems::update_score);
  }
}

pub mod components {
  use bevy::prelude::{Component, Entity};

  #[derive(Component)]
  pub struct Scoreboard;

  #[derive(Debug, Component)]
  pub struct Score(pub usize);

  #[derive(Debug, Component)]
  pub struct Place(pub f32);

  #[derive(Debug, Component)]
  pub struct ScoreEntity(pub Entity);

  #[derive(Debug, Component)]
  pub struct ScoreValue;

  #[derive(Debug, Component)]
  pub struct Name(pub String);
}

pub mod events {
  pub struct ScoreUpdate;
}

pub mod utils {
  use super::{
    components::{Name, Place, Score, ScoreValue},
    styles,
  };
  use bevy::prelude::{BuildChildren, Color, Commands, Entity, NodeBundle, TextBundle};

  pub fn spawn_score(commands: &mut Commands, score: usize, name: String, color: Color) -> Entity {
    commands
      .spawn((
        Score(score),
        Name(name.clone()),
        Place(styles::SCORE_HEIGHT),
        NodeBundle {
          background_color: styles::SCORE_BACKGROUND.into(),
          style: styles::SCORE,
          ..Default::default()
        },
      ))
      .with_children(|parent| {
        parent.spawn(TextBundle::from_section(name, styles::text(color)));
        parent.spawn((
          ScoreValue,
          TextBundle::from_section(score.to_string(), styles::text(color)),
        ));
      })
      .id()
  }
}
