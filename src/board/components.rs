use bevy::{
  prelude::{Bundle, Color, Component, Transform, Visibility},
  sprite::{Sprite, SpriteBundle},
};

use super::{utils::get_board_position, CELL_SIZE_VEC};

#[derive(Debug, Component)]
pub struct Board;

#[derive(Debug, Component)]
pub struct BoardSprite;

#[derive(Debug, Component)]
pub struct Cell;

#[derive(Debug, Component)]
pub struct CellCollider;

#[derive(Debug, Component)]
pub struct DyingCell;

#[derive(Debug, Component)]
pub struct RandomCellPosition;

#[derive(Bundle)]
pub struct CellBundle {
  cell: Cell,
  collider: CellCollider,
  #[bundle]
  sprite_bundle: SpriteBundle,
}

impl CellBundle {
  pub fn new(color: Color, x: f32, y: f32) -> Self {
    Self {
      cell: Cell,
      collider: CellCollider,
      sprite_bundle: SpriteBundle {
        sprite: Sprite {
          color,
          custom_size: Some(CELL_SIZE_VEC),
          ..Default::default()
        },
        visibility: Visibility::Hidden,
        transform: Transform::from_translation(get_board_position(x, y)),
        ..Default::default()
      },
    }
  }
}
