use bevy::{
  prelude::{Camera2dBundle, Commands, Query, Transform, With},
  window::{PrimaryWindow, Window},
};

pub(super) fn spawn_camera(
  mut commands: Commands,
  window: Query<&Window, With<PrimaryWindow>>,
) {
  let window = window.get_single().unwrap();
  commands.spawn(Camera2dBundle {
    transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
    ..Default::default()
  });
}
