use super::{
  components::{DirectionQueue, Player},
  events::RespawnPlayer,
  INITIAL_PLAYER_LENGTH, PLAYER_COLOR,
};
use crate::{
  attributes::components::MoveCooldown,
  snake::{
    components::{Direction, Living, Revive, Snake, SnakeBundle, SnakeConfig},
    events::Serpentine,
  },
  tetris::{components::TetrisBlock, events::TetrisMove},
};
use bevy::prelude::{
  Commands, Entity, EventReader, EventWriter, Input, KeyCode, Query, Res, With, Without,
};

pub(super) fn spawn(mut commands: Commands) {
  let player = (
    Player,
    DirectionQueue::default(),
    SnakeBundle::new(
      &mut commands,
      SnakeConfig {
        name: "Player".to_string(),
        color: PLAYER_COLOR,
        tail_length: INITIAL_PLAYER_LENGTH,
        ..Default::default()
      },
    ),
  );
  commands.spawn(player);
}

pub(super) fn respawn(
  mut commands: Commands,
  mut respawn_reader: EventReader<RespawnPlayer>,
  mut q_player: Query<Entity, (With<Player>, With<Snake>, Without<Living>)>,
) {
  for _ in respawn_reader.iter() {
    let Ok(player) = q_player.get_single_mut() else {return};
    commands.entity(player).insert(Revive);
  }
}

pub(super) fn queue_snake_input(
  keyboard_input: Res<Input<KeyCode>>,
  mut q_player: Query<(&mut Direction, &mut DirectionQueue), (With<Player>, With<Snake>)>,
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

pub(super) fn iter_snake_input(
  mut serpentine_reader: EventReader<Serpentine>,
  mut q_player: Query<(&mut Direction, &mut DirectionQueue), (With<Player>, With<Snake>)>,
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

pub(super) fn tetris_input(
  mut move_writer: EventWriter<TetrisMove>,
  q_player: Query<(Entity, &MoveCooldown), (With<Player>, With<TetrisBlock>)>,
  keyboard_input: Res<Input<KeyCode>>,
) {
  let Ok((player, move_cooldown)) = q_player.get_single() else {return};
  if move_cooldown.0.finished() {
    if keyboard_input.pressed(KeyCode::A) {
      move_writer.send(TetrisMove::Left(player));
    } else if keyboard_input.pressed(KeyCode::D) {
      move_writer.send(TetrisMove::Right(player));
    }
  }
}
