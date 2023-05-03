use crate::{
  attributes::components::{ColorBundle, Solid, SpeedBundle},
  board::{components::CellBundle, CELL_SIZE},
  scoreboard::{components::ScoreEntity, utils::spawn_score},
};
use bevy::prelude::{Bundle, Color, Commands, Component, Deref, Entity, Vec3};
use rand::Rng;
use std::collections::VecDeque;

use super::utils::SNAKE_NAMES;

pub struct SnakeConfig {
  pub name: String,
  pub x: f32,
  pub y: f32,
  pub speed: f32,
  pub color: Color,
  pub brightness: f32,
  pub direction: Direction,
  pub tail_length: usize,
  pub score: Option<Entity>,
}

impl Default for SnakeConfig {
  fn default() -> Self {
    Self {
      name: SNAKE_NAMES[rand::thread_rng().gen_range(0..50)].to_string(),
      color: Color::WHITE,
      brightness: 0.,
      tail_length: 4,
      speed: 1.,
      direction: Direction::default(),
      x: 0.,
      y: 0.,
      score: None,
    }
  }
}

#[derive(Bundle)]
pub struct SnakeBundle {
  snake: Snake,
  score: ScoreEntity,
  direction: Direction,
  body: SnakeBody,
  living: Living,
  #[bundle]
  color_bundle: ColorBundle,
  #[bundle]
  speed_bundle: SpeedBundle,
  #[bundle]
  cell_bundle: CellBundle,
}

impl SnakeBundle {
  pub fn new(commands: &mut Commands, config: SnakeConfig) -> Self {
    let score = config.score.unwrap_or_else(|| {
      spawn_score(
        commands,
        config.tail_length as i32,
        config.name,
        config.color,
      )
    });
    Self {
      snake: Snake,
      score: ScoreEntity(score),
      direction: config.direction,
      body: SnakeBody::new(
        commands,
        config.color,
        config.x,
        config.y,
        config.tail_length,
      ),
      living: Living,
      color_bundle: ColorBundle::new(config.color, config.brightness),
      speed_bundle: SpeedBundle::new(config.speed),
      cell_bundle: CellBundle::new(config.color, config.x, config.y),
    }
  }
}

#[derive(Debug, Component)]
pub struct Snake;

#[derive(Debug, Component)]
pub struct Snakified;

#[derive(Debug, Component)]
pub struct Living;

#[derive(Debug, Component)]
pub struct Revive;

#[derive(Debug, Component, Default)]
pub struct Satiety(pub u32);

#[derive(Debug, Component)]
pub struct Nourishment(pub u32);

#[derive(Debug, Component)]
pub struct Hunger(pub u32);

#[derive(Debug, Component, Default, PartialEq, Clone, Copy)]
pub enum Direction {
  Bottom,
  Left,
  #[default]
  Right,
  Top,
}

impl Direction {
  pub fn opposite(&self) -> Self {
    use Direction::*;
    match *self {
      Bottom => Top,
      Left => Right,
      Right => Left,
      Top => Bottom,
    }
  }

  pub fn xy(&self, x: f32, y: f32) -> (f32, f32) {
    match self {
      Direction::Bottom => (0., -y),
      Direction::Right => (x, 0.),
      Direction::Top => (0., y),
      Direction::Left => (-x, 0.),
    }
  }
}

#[derive(Debug, Component)]
pub struct SnakeSegment;

#[derive(Bundle)]
pub struct SnakeSegmentBundle {
  segment: SnakeSegment,
  solid: Solid,
  #[bundle]
  cell_bundle: CellBundle,
}

impl SnakeSegmentBundle {
  pub(super) fn new(color: Color, x: f32, y: f32) -> Self {
    Self {
      segment: SnakeSegment,
      solid: Solid,
      cell_bundle: CellBundle::new(color, x, y),
    }
  }
}

#[derive(Debug, Component, Deref)]
pub struct SnakeBody(VecDeque<Entity>);

impl SnakeBody {
  pub fn new(commands: &mut Commands, color: Color, x: f32, y: f32, tail_length: usize) -> Self {
    Self(
      (1..=tail_length)
        .map(|i| {
          commands
            .spawn(SnakeSegmentBundle::new(color, x - CELL_SIZE * i as f32, y))
            .id()
        })
        .collect(),
    )
  }

  pub fn head(&self) -> Option<Entity> {
    self.0.front().copied()
  }

  pub(super) fn tail(&self) -> Option<Entity> {
    self.0.back().copied()
  }

  pub(super) fn push_head(&mut self, entity: Entity) {
    self.0.push_front(entity)
  }

  pub(super) fn push_tail(&mut self, entity: Entity) {
    self.0.push_back(entity)
  }

  pub(super) fn pop_tail(&mut self) -> Option<Entity> {
    self.0.pop_back()
  }
}

#[derive(Debug, Component, Default)]
pub struct Seeker(pub Vec3);
