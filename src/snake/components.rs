use crate::board::{utils::create_cell_bundle, CELL_SIZE};
use bevy::{
  prelude::{
    BuildChildren, Bundle, Color, Commands, Component, Deref, DerefMut, Entity, SpriteBundle,
  },
  time::{Timer, TimerMode},
};
use std::{collections::VecDeque, time::Duration};

pub struct SnakeConfig {
  pub x: f32,
  pub y: f32,
  pub serpentine_duration_ms: u64,
  pub color: Color,
  pub direction: Direction,
  pub tail_length: usize,
}

impl Default for SnakeConfig {
  fn default() -> Self {
    Self {
      color: Color::WHITE,
      tail_length: 4,
      serpentine_duration_ms: 100,
      direction: Direction::default(),
      x: 0.,
      y: 0.,
    }
  }
}

#[derive(Bundle)]
pub struct SnakeBundle {
  snake: Snake,
  direction: Direction,
  body: SnakeBody,
  living: Living,
  speed: Speed,
  #[bundle]
  sprite_bundle: SpriteBundle,
}

impl SnakeBundle {
  pub fn new(commands: &mut Commands, board: Entity, config: SnakeConfig) -> Self {
    Self {
      snake: Snake,
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
      speed: Speed(Timer::new(
        Duration::from_millis(config.serpentine_duration_ms),
        TimerMode::Repeating,
      )),
      sprite_bundle: create_cell_bundle(config.color, config.x, config.y),
    }
  }
}

#[derive(Debug, Component)]
pub struct Snake;

#[derive(Debug, Component)]
pub struct Living;

#[derive(Debug, Component, DerefMut, Deref)]
pub struct Speed(Timer);

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

#[derive(Debug, Component)]
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

  pub fn head(&self) -> Option<Entity> {
    self.0.front().copied()
  }

  pub(super) fn tail(&self) -> Option<Entity> {
    self.0.back().copied()
  }

  pub(super) fn push_head(&mut self, entity: Entity) {
    self.0.push_front(entity)
  }

  pub(super) fn extend_tail(&mut self, tail_segments: impl IntoIterator<Item = Entity>) {
    self.0.extend(tail_segments);
  }

  pub(super) fn pop_tail(&mut self) -> Option<Entity> {
    self.0.pop_back()
  }
}
