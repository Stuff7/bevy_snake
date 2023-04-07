use super::{CELL_SIZE, CELL_SIZE_VEC, HALF_CELL_SIZE};
use bevy::{
  prelude::{Color, Transform, Vec3},
  sprite::{Sprite, SpriteBundle},
};

pub fn create_cell_bundle(color: Color, x: f32, y: f32) -> SpriteBundle {
  SpriteBundle {
    sprite: Sprite {
      color,
      custom_size: Some(CELL_SIZE_VEC),
      ..Default::default()
    },
    transform: Transform::from_translation(get_board_position(x, y)),
    ..Default::default()
  }
}

pub fn get_board_position(x: f32, y: f32) -> Vec3 {
  Vec3::new(
    ((x / CELL_SIZE) as u32 * CELL_SIZE as u32) as f32 + HALF_CELL_SIZE,
    ((y / CELL_SIZE) as u32 * CELL_SIZE as u32) as f32 + HALF_CELL_SIZE,
    0.,
  )
}
