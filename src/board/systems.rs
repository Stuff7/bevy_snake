use bevy::{
  prelude::{BuildChildren, Commands, Query, SpatialBundle, Sprite, Transform, Vec2, With},
  sprite::SpriteBundle,
  window::{PrimaryWindow, Window},
};

use super::{components::Board, BOARD_COLOR, BOARD_SIZE};

pub(super) fn spawn(mut commands: Commands, window: Query<&Window, With<PrimaryWindow>>) {
  let window = window.get_single().unwrap();

  let board = commands
    .spawn(SpriteBundle {
      sprite: Sprite {
        color: BOARD_COLOR,
        custom_size: Some(Vec2::new(BOARD_SIZE, BOARD_SIZE)),
        ..Default::default()
      },
      ..Default::default()
    })
    .id();

  commands
    .spawn((
      Board,
      SpatialBundle::from_transform(Transform::from_xyz(
        window.width() / 2. - BOARD_SIZE / 2.,
        0.,
        0.,
      )),
    ))
    .add_child(board);
}
