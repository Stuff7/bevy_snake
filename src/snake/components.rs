use crate::world::{styles::create_cell_bundle, CELL_WIDTH};

use super::PLAYER_COLOR;
use bevy::prelude::{Commands, Component, Entity, Input, KeyCode};
use std::collections::VecDeque;

#[derive(Component)]
pub struct Player;

#[derive(Component, Default, PartialEq, Clone, Copy)]
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

  pub(super) fn next_from_input(&self, keyboard_input: &Input<KeyCode>) -> Option<Self> {
    if self != &Direction::Bottom && keyboard_input.pressed(KeyCode::W) {
      Some(Direction::Top)
    } else if self != &Direction::Right && keyboard_input.pressed(KeyCode::A) {
      Some(Direction::Left)
    } else if self != &Direction::Top && keyboard_input.pressed(KeyCode::S) {
      Some(Direction::Bottom)
    } else if self != &Direction::Left && keyboard_input.pressed(KeyCode::D) {
      Some(Direction::Right)
    } else {
      None
    }
  }
}

#[derive(Component, Default)]
pub struct DirectionQueue {
  pub(super) previous: Direction,
  pub(super) current: Direction,
  pub(super) next: Option<Direction>,
}

impl DirectionQueue {
  pub(super) fn advance(&mut self) {
    if self.current == self.previous {
      if let Some(next_direction) = self.next.take() {
        self.current = next_direction;
      }
    }
    self.previous = self.current;
  }
}

#[derive(Debug, Component, Clone, Copy)]
pub struct SnakeHead {
  pub(super) x: f32,
  pub(super) y: f32,
}

#[derive(Debug, Component)]
pub struct SnakeSegment;

impl SnakeSegment {
  pub(super) fn spawn(commands: &mut Commands, x: f32, y: f32) -> Entity {
    commands
      .spawn((SnakeSegment, create_cell_bundle(PLAYER_COLOR, x, y)))
      .id()
  }
}

#[derive(Debug, Component)]
pub struct SnakeBody(VecDeque<Entity>);

impl SnakeBody {
  pub(super) fn new(commands: &mut Commands, head: SnakeHead, tail_length: usize) -> Self {
    let entity = commands
      .spawn((
        head,
        SnakeSegment,
        create_cell_bundle(PLAYER_COLOR, head.x, head.y),
      ))
      .id();
    let mut body = VecDeque::from([entity]);

    body.extend(
      (1..=tail_length)
        .map(|i| SnakeSegment::spawn(commands, head.x - CELL_WIDTH * i as f32, head.y)),
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
