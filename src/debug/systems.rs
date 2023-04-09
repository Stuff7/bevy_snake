use crate::{
  board::components::Board,
  food::components::Food,
  player::{components::Player, events::RespawnPlayer},
  scoreboard::components::Name,
  snake::{
    components::Snake,
    events::{BodySizeChange, SnakeSizeChange},
  },
};
use bevy::prelude::{Entity, EventWriter, Input, KeyCode, Query, Res, Transform, With};

pub(super) fn god_mode(
  mut respawn_player_writer: EventWriter<RespawnPlayer>,
  mut size_change_writer: EventWriter<SnakeSizeChange>,
  q_player: Query<Entity, With<Player>>,
  keyboard_input: Res<Input<KeyCode>>,
) {
  use BodySizeChange::*;
  if keyboard_input.just_pressed(KeyCode::E) {
    let Ok(player) = q_player.get_single() else { return; };
    size_change_writer.send((player, Grow));
  } else if keyboard_input.just_pressed(KeyCode::Q) {
    let Ok(player) = q_player.get_single() else { return; };
    size_change_writer.send((player, Shrink));
  } else if keyboard_input.just_pressed(KeyCode::R) {
    if q_player.get_single().is_ok() {
      return;
    }
    respawn_player_writer.send(RespawnPlayer);
  }
}

pub(super) fn move_board(
  mut q_board: Query<&mut Transform, With<Board>>,
  keyboard_input: Res<Input<KeyCode>>,
) {
  let Ok(mut board) = q_board.get_single_mut() else {return};
  if keyboard_input.pressed(KeyCode::Left) {
    board.translation.x -= 1.;
  } else if keyboard_input.pressed(KeyCode::Right) {
    board.translation.x += 1.;
  } else if keyboard_input.pressed(KeyCode::Up) {
    board.translation.y += 1.;
  } else if keyboard_input.pressed(KeyCode::Down) {
    board.translation.y -= 1.;
  }
}

pub(super) fn print_debug_info(
  keyboard_input: Res<Input<KeyCode>>,
  q_entity: Query<Entity>,
  q_snake: Query<&Name, With<Snake>>,
  q_food: Query<Entity, With<Food>>,
) {
  if keyboard_input.just_pressed(KeyCode::P) {
    let debug = [
      "=== === === DEBUG === === ===",
      &format!("Entity Count: {}", q_entity.iter().count()),
      &format!("Snakes: {:#?}", q_snake.iter().collect::<Vec<_>>()),
      &format!("Food: {:#?}", q_food.get_single()),
    ]
    .join("\n");
    println!("{debug}");
  }
}
