use crate::{
  food::components::Food,
  player::{components::Player, events::RespawnPlayer},
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
    size_change_writer.send((player, Grow(1)));
  } else if keyboard_input.just_pressed(KeyCode::Q) {
    let Ok(player) = q_player.get_single() else { return; };
    size_change_writer.send((player, Shrink(1)));
  } else if keyboard_input.just_pressed(KeyCode::R) {
    respawn_player_writer.send(RespawnPlayer);
  }
}

pub(super) fn print_debug_info(
  keyboard_input: Res<Input<KeyCode>>,
  q_entity: Query<Entity>,
  q_snake: Query<&Transform, With<Snake>>,
  q_food: Query<Entity, With<Food>>,
) {
  if keyboard_input.just_pressed(KeyCode::P) {
    let debug = [
      "=== === === DEBUG === === ===",
      &format!("Entity Count: {}", q_entity.iter().count()),
      &format!(
        "Snakes: {:#?}",
        q_snake.iter().map(|t| t.translation).collect::<Vec<_>>()
      ),
      &format!("Food: {:#?}", q_food.get_single()),
    ]
    .join("\n");
    println!("{debug}");
  }
}
