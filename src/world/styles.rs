use super::{CELL_HEIGHT, CELL_SIZE, CELL_WIDTH};

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
    transform: Transform::from_xyz(
      ((x / CELL_WIDTH) as u32 * CELL_WIDTH as u32) as f32,
      ((y / CELL_HEIGHT) as u32 * CELL_HEIGHT as u32) as f32,
      0.,
    ),
    ..Default::default()
  }
}
