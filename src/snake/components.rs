use crate::board::{utils::create_cell_bundle, CELL_SIZE};
use bevy::prelude::{
  BuildChildren, Bundle, Color, Commands, Component, Entity, SpriteBundle, Vec3,
};
use std::collections::VecDeque;

#[derive(Bundle)]
pub struct SnakeBundle {
  snake: Snake,
  direction: Direction,
  body: SnakeBody,
  living: Living,
  #[bundle]
  sprite_bundle: SpriteBundle,
}

impl SnakeBundle {
  pub fn new(
    commands: &mut Commands,
    board: Entity,
    x: f32,
    y: f32,
    color: Color,
    direction: Direction,
    tail_length: usize,
  ) -> Self {
    Self {
      snake: Snake,
      sprite_bundle: create_cell_bundle(color, x, y),
      direction,
      body: SnakeBody::new(commands, board, color, x, y, tail_length),
      living: Living,
    }
  }
}

#[derive(Debug, Component)]
pub struct Snake;

#[derive(Debug, Component)]
pub struct Living;

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

pub fn snake_crashed<H: Iterator<Item = (Entity, Vec3)>, B: Iterator<Item = Vec3>>(
  mut head_iter: H,
  mut body_iter: B,
  snake_entity: Entity,
  snake_head: Vec3,
) -> bool {
  head_iter.any(|(entity, head)| {
    if entity == snake_entity {
      return false;
    }
    head.distance(snake_head) < CELL_SIZE
  }) || body_iter.any(|segment| segment.distance(snake_head) < CELL_SIZE)
}
