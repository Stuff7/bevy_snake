use bevy::prelude::{Bundle, Color, Component, Entity};

use crate::{
  attributes::components::{ColorBundle, Solid, SpeedBundle},
  board::components::CellBundle,
  scoreboard::components::ScoreEntity,
};

#[derive(Debug, Component)]
pub struct TetrisBlock;

#[derive(Debug, Component)]
pub struct Placed;

#[derive(Debug, Component)]
pub struct FreeFall;

#[derive(Debug, Component)]
pub struct BlockPart;

#[derive(Bundle)]
pub struct BlockPartBundle {
  part: BlockPart,
  solid: Solid,
  #[bundle]
  cell_bundle: CellBundle,
}

impl BlockPartBundle {
  pub fn new(color: Color, x: f32, y: f32) -> Self {
    Self {
      part: BlockPart,
      solid: Solid,
      cell_bundle: CellBundle::new(color, x, y),
    }
  }
}

#[derive(Debug, Component)]
pub struct BlockParts(pub Vec<Entity>);

#[derive(Debug, Component)]
pub struct Tetrified;

#[derive(Bundle)]
pub struct TetrisBlockBundle {
  block: TetrisBlock,
  parts: BlockParts,
  score: ScoreEntity,
  #[bundle]
  color_bundle: ColorBundle,
  #[bundle]
  speed_bundle: SpeedBundle,
}

impl TetrisBlockBundle {
  pub fn new(parts: &[Entity], color_bundle: ColorBundle, score: Entity, speed: f32) -> Self {
    Self {
      block: TetrisBlock,
      parts: BlockParts(parts.into()),
      score: ScoreEntity(score),
      color_bundle,
      speed_bundle: SpeedBundle::new(speed),
    }
  }
}
