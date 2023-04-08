use bevy::{
  prelude::{BuildChildren, Commands, Query, SpatialBundle, Sprite, Vec2, With},
  sprite::SpriteBundle,
  window::{PrimaryWindow, Window},
};

use super::{components::Board, BOARD_COLOR, CELL_SIZE};

pub(super) fn spawn(mut commands: Commands, window: Query<&Window, With<PrimaryWindow>>) {
  let window = window.get_single().unwrap();
  let width = (window.width() / CELL_SIZE).floor() * CELL_SIZE;
  let height = (window.height() / CELL_SIZE).floor() * CELL_SIZE;

  let board = commands
    .spawn(SpriteBundle {
      sprite: Sprite {
        color: BOARD_COLOR,
        custom_size: Some(Vec2::new(width, height)),
        ..Default::default()
      },
      ..Default::default()
    })
    .id();

  commands
    .spawn((Board, SpatialBundle::default()))
    .add_child(board);
}
