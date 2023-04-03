use super::CELL_SIZE;

use bevy::{
  prelude::{Color, Transform},
  sprite::{Sprite, SpriteBundle},
};

pub fn create_cell_bundle(color: Color, x: f32, y: f32) -> SpriteBundle {
  SpriteBundle {
    sprite: Sprite {
      color,
      custom_size: Some(CELL_SIZE),
      ..Default::default()
    },
    transform: Transform::from_xyz(x, y, 0.),
    ..Default::default()
  }
}
