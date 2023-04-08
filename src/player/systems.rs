use super::{
  components::{DirectionQueue, Player},
  events::RespawnPlayer,
  INITIAL_PLAYER_LENGTH, PLAYER_COLOR,
};
use crate::snake::{
  components::{Direction, SnakeBundle},
  events::Serpentine,
};
use bevy::prelude::{Commands, EventReader, EventWriter, Input, KeyCode, Query, Res, With};

pub(super) fn startup(mut respawn_writer: EventWriter<RespawnPlayer>) {
  respawn_writer.send(RespawnPlayer);
}

pub(super) fn spawn(
  mut commands: Commands,
  mut respawn_reader: EventReader<RespawnPlayer>,
  q_player: Query<(), With<Player>>,
) {
  for _ in respawn_reader.iter() {
    if q_player.get_single().is_ok() {
      return;
    }
    let player = (
      Player,
      DirectionQueue::default(),
      SnakeBundle::new(
        &mut commands,
        0.,
        0.,
        PLAYER_COLOR,
        Direction::default(),
        INITIAL_PLAYER_LENGTH,
      ),
    );

    commands.spawn(player);
  }
}

pub(super) fn queue_input(
  keyboard_input: Res<Input<KeyCode>>,
  mut q_player: Query<(&mut Direction, &mut DirectionQueue), With<Player>>,
) {
  let Ok((mut direction, mut direction_queue)) = q_player.get_single_mut() else { return; };

  use Direction::*;
  let new_direction = if keyboard_input.pressed(KeyCode::W) {
    Top
  } else if keyboard_input.pressed(KeyCode::A) {
    Left
  } else if keyboard_input.pressed(KeyCode::S) {
    Bottom
  } else if keyboard_input.pressed(KeyCode::D) {
    Right
  } else {
    return;
  };

  if new_direction == direction.opposite() {
    return;
  }

  if *direction == direction_queue.previous {
    *direction = new_direction;
  } else {
    direction_queue.next = Some(new_direction);
  }
}

pub(super) fn iter_input(
  mut serpentine_reader: EventReader<Serpentine>,
  mut q_player: Query<(&mut Direction, &mut DirectionQueue), With<Player>>,
) {
  let Ok((mut direction, mut direction_queue)) = q_player.get_single_mut() else { return; };
  for _ in serpentine_reader.iter() {
    if *direction == direction_queue.previous {
      if let Some(next_direction) = direction_queue.next.take() {
        *direction = next_direction;
      }
    }
    direction_queue.previous = *direction;
  }
}
