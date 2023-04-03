use crate::world::styles::create_cell_bundle;

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

impl SnakeHead {
  pub(super) fn spawn_and_create(
    commands: &mut Commands,
    x: f32,
    y: f32,
  ) -> (Entity, Self) {
    (Self::spawn(commands, x, y), Self { x, y })
  }

  pub(super) fn spawn(commands: &mut Commands, x: f32, y: f32) -> Entity {
    let mut entity_commands = commands.spawn_empty();
    entity_commands.insert(create_cell_bundle(PLAYER_COLOR, x, y));
    entity_commands.id()
  }
}

#[derive(Debug, Component)]
pub struct SnakeBody(VecDeque<Entity>);

impl SnakeBody {
  pub(super) fn new(entity: Entity) -> Self {
    Self(vec![entity].into())
  }

  pub(super) fn head(&self) -> Entity {
    self
      .0
      .front()
      .copied()
      .expect("SnakeBody should always have a head")
  }

  pub(super) fn push_head(&mut self, entity: Entity) {
    self.0.push_front(entity)
  }

  pub(super) fn pop_tail(&mut self) -> Option<Entity> {
    (self.0.len() > 1).then(|| self.0.pop_back()).flatten()
  }
}
