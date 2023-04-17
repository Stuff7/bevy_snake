use super::{
  components::{DirectionQueue, Player},
  events::RespawnPlayer,
  INITIAL_PLAYER_LENGTH, PLAYER_COLOR,
};
use crate::{
  board::{components::Board, resources::GameBoard},
  color::components::Brightness,
  snake::{
    components::{Direction, Living, SnakeBundle, SnakeConfig, Speed},
    events::Serpentine,
    utils::revive_snake,
  },
};
use bevy::prelude::{
  BuildChildren, Commands, Entity, EventReader, Input, KeyCode, Query, Res, Transform, Visibility,
  With, Without,
};

pub(super) fn spawn(mut commands: Commands, q_board: Query<Entity, With<Board>>) {
  let Ok(board) = q_board.get_single() else {return};
  let player = (
    Player,
    DirectionQueue::default(),
    SnakeBundle::new(
      &mut commands,
      board,
      SnakeConfig {
        name: "Player".to_string(),
        color: PLAYER_COLOR,
        tail_length: INITIAL_PLAYER_LENGTH,
        ..Default::default()
      },
    ),
  );
  let player = commands.spawn(player).id();
  commands.entity(board).add_child(player);
}

pub(super) fn respawn(
  mut commands: Commands,
  mut respawn_reader: EventReader<RespawnPlayer>,
  mut q_player: Query<
    (
      Entity,
      &mut Visibility,
      &mut Transform,
      &mut Speed,
      &mut Brightness,
    ),
    (With<Player>, Without<Living>),
  >,
  game_board: Res<GameBoard>,
) {
  for _ in respawn_reader.iter() {
    let Ok((player, mut visibility, mut transform, mut speed, mut brightness)) = q_player.get_single_mut() else {return};
    revive_snake(
      &mut commands,
      (
        player,
        &mut visibility,
        &mut transform,
        &mut speed,
        &mut brightness,
      ),
      &game_board,
    );
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
  for snake in &mut serpentine_reader {
    let Ok((mut direction, mut direction_queue)) = q_player.get_mut(snake.0) else {continue};
    let should_take_next = *direction == direction_queue.previous;
    direction_queue.previous = *direction;
    let Some(next_direction) = direction_queue.next.take() else {continue};
    if should_take_next && next_direction != direction.opposite() {
      *direction = next_direction;
    }
  }
}
