use super::{
  components::{DirectionQueue, Player},
  events::RespawnPlayer,
  INITIAL_PLAYER_LENGTH, PLAYER_COLOR,
};
use crate::snake::{
  components::{Direction, SnakeBundle},
  events::Serpentine,
};
use bevy::{
  prelude::{Commands, EventReader, Input, KeyCode, Query, Res, With},
  window::{PrimaryWindow, Window},
};

pub(super) fn spawn(mut commands: Commands, window: Query<&Window, With<PrimaryWindow>>) {
  spawn_player(&mut commands, window.get_single().unwrap());
}

pub(super) fn respawn(
  mut commands: Commands,
  mut respawn_reader: EventReader<RespawnPlayer>,
  window: Query<&Window, With<PrimaryWindow>>,
  player_query: Query<(), With<Player>>,
) {
  for _ in respawn_reader.iter() {
    if player_query.get_single().is_ok() {
      return;
    }
    spawn_player(&mut commands, window.get_single().unwrap());
  }
}

pub(super) fn queue_input(
  keyboard_input: Res<Input<KeyCode>>,
  mut player_query: Query<(&mut Direction, &mut DirectionQueue), With<Player>>,
) {
  let Ok((mut direction, mut direction_queue)) = player_query.get_single_mut() else { return; };

  use Direction::*;
  let new_direction = if *direction != Bottom && keyboard_input.pressed(KeyCode::W) {
    Top
  } else if *direction != Right && keyboard_input.pressed(KeyCode::A) {
    Left
  } else if *direction != Top && keyboard_input.pressed(KeyCode::S) {
    Bottom
  } else if *direction != Left && keyboard_input.pressed(KeyCode::D) {
    Right
  } else {
    return;
  };

  if *direction == direction_queue.previous {
    *direction = new_direction;
  } else {
    direction_queue.next = Some(new_direction);
  }
}

pub(super) fn iter_input(
  mut serpentine_reader: EventReader<Serpentine>,
  mut player_query: Query<(&mut Direction, &mut DirectionQueue), With<Player>>,
) {
  let Ok((mut direction, mut direction_queue)) = player_query.get_single_mut() else { return; };
  for _ in serpentine_reader.iter() {
    if *direction == direction_queue.previous {
      if let Some(next_direction) = direction_queue.next.take() {
        *direction = next_direction;
      }
    }
    direction_queue.previous = *direction;
  }
}

fn spawn_player(commands: &mut Commands, window: &Window) {
  let player = (
    Player,
    DirectionQueue::default(),
    SnakeBundle::new(
      commands,
      window.width() / 2.,
      window.height() / 2.,
      PLAYER_COLOR,
      Direction::default(),
      INITIAL_PLAYER_LENGTH,
    ),
  );

  commands.spawn(player);
}
