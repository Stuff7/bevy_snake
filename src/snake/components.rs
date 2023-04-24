use crate::{
  attributes::components::{BaseColor, Brightness, SpeedBundle},
  board::{utils::create_cell_bundle, CELL_SIZE},
  scoreboard::{components::ScoreEntity, utils::spawn_score},
};
use bevy::prelude::{
  BuildChildren, Bundle, Color, Commands, Component, Deref, Entity, SpriteBundle, Vec3,
};
use rand::Rng;
use std::collections::VecDeque;

use super::utils::SNAKE_NAMES;

pub struct SnakeConfig {
  pub name: String,
  pub x: f32,
  pub y: f32,
  pub speed: f32,
  pub color: Color,
  pub direction: Direction,
  pub tail_length: usize,
}

impl Default for SnakeConfig {
  fn default() -> Self {
    Self {
      name: SNAKE_NAMES[rand::thread_rng().gen_range(0..50)].to_string(),
      color: Color::WHITE,
      tail_length: 4,
      speed: 1.,
      direction: Direction::default(),
      x: 0.,
      y: 0.,
    }
  }
}

#[derive(Bundle)]
pub struct SnakeBundle {
  snake: Snake,
  color: BaseColor,
  brightness: Brightness,
  score: ScoreEntity,
  direction: Direction,
  body: SnakeBody,
  living: Living,
  #[bundle]
  speed_bundle: SpeedBundle,
  #[bundle]
  sprite_bundle: SpriteBundle,
}

impl SnakeBundle {
  pub fn new(commands: &mut Commands, board: Entity, config: SnakeConfig) -> Self {
    let score = spawn_score(commands, config.tail_length, config.name, config.color);
    Self {
      snake: Snake,
      color: BaseColor(config.color),
      brightness: Brightness::default(),
      score: ScoreEntity(score),
      direction: config.direction,
      body: SnakeBody::new(
        commands,
        board,
        config.color,
        config.x,
        config.y,
        config.tail_length,
      ),
      living: Living,
      speed_bundle: SpeedBundle::new(config.speed),
      sprite_bundle: create_cell_bundle(config.color, config.x, config.y),
    }
  }
}

#[derive(Debug, Component)]
pub struct Snake;

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

impl SnakeSegment {
  pub(super) fn spawn(
    commands: &mut Commands,
    board: Entity,
    color: Color,
    x: f32,
    y: f32,
  ) -> Entity {
    let segment = commands
      .spawn((SnakeSegment, create_cell_bundle(color, x, y)))
      .id();
    commands.entity(board).add_child(segment);
    segment
  }
}

#[derive(Debug, Component, Deref)]
pub struct SnakeBody(VecDeque<Entity>);

impl SnakeBody {
  pub fn new(
    commands: &mut Commands,
    board: Entity,
    color: Color,
    x: f32,
    y: f32,
    tail_length: usize,
  ) -> Self {
    Self(
      (1..=tail_length)
        .map(|i| SnakeSegment::spawn(commands, board, color, x - CELL_SIZE * i as f32, y))
        .collect(),
    )
  }

  pub fn len(&self) -> usize {
    self.0.len()
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
