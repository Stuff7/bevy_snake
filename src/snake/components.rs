use crate::world::{styles::create_cell_bundle, CELL_WIDTH};
use bevy::prelude::{Color, Commands, Component, Entity};
use std::collections::VecDeque;

#[derive(Debug, Component, Default, PartialEq, Clone, Copy)]
pub enum Direction {
  Bottom,
  Left,
  #[default]
  Right,
  Top,
}

impl Direction {
  pub(super) fn opposite(&self) -> Self {
    use Direction::*;
    match *self {
      Bottom => Top,
      Left => Right,
      Right => Left,
      Top => Bottom,
    }
  }

  pub(super) fn xy(&self, x: f32, y: f32) -> (f32, f32) {
    match self {
      Direction::Bottom => (0., -y),
      Direction::Right => (x, 0.),
      Direction::Top => (0., y),
      Direction::Left => (-x, 0.),
    }
  }
}

#[derive(Debug, Component)]
pub struct SnakeHead;

#[derive(Debug, Component)]
pub struct SnakeSegment;

impl SnakeSegment {
  pub(super) fn spawn(commands: &mut Commands, color: Color, x: f32, y: f32) -> Entity {
    commands
      .spawn((SnakeSegment, create_cell_bundle(color, x, y)))
      .id()
  }
}

#[derive(Debug, Component)]
pub struct SnakeBody(VecDeque<Entity>);

impl SnakeBody {
  pub fn new(commands: &mut Commands, color: Color, x: f32, y: f32, tail_length: usize) -> Self {
    let entity = commands
      .spawn((SnakeHead, SnakeSegment, create_cell_bundle(color, x, y)))
      .id();
    let mut body = VecDeque::from([entity]);

    body.extend(
      (1..=tail_length).map(|i| SnakeSegment::spawn(commands, color, x - CELL_WIDTH * i as f32, y)),
    );

    Self(body)
  }

  pub(super) fn head(&self) -> Entity {
    self
      .0
      .front()
      .copied()
      .expect("SnakeBody should always have a head")
  }

  pub(super) fn tail(&self) -> Entity {
    self.0.back().copied().unwrap_or_else(|| self.head())
  }

  pub(super) fn push_head(&mut self, entity: Entity) {
    self.0.push_front(entity)
  }

  pub(super) fn extend_tail(&mut self, tail_segments: impl IntoIterator<Item = Entity>) {
    self.0.extend(tail_segments);
  }

  pub(super) fn pop_tail(&mut self) -> Option<Entity> {
    (self.0.len() > 1).then(|| self.0.pop_back()).flatten()
  }
}
