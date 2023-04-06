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
  player_query: Query<Entity, With<Player>>,
  keyboard_input: Res<Input<KeyCode>>,
) {
  use BodySizeChange::*;
  if keyboard_input.just_pressed(KeyCode::E) {
    let Ok(player) = player_query.get_single() else { return; };
    size_change_writer.send((player, Grow(1)));
  } else if keyboard_input.just_pressed(KeyCode::Q) {
    let Ok(player) = player_query.get_single() else { return; };
    size_change_writer.send((player, Shrink(1)));
  } else if keyboard_input.just_pressed(KeyCode::R) {
    respawn_player_writer.send(RespawnPlayer);
  }
}

pub(super) fn print_debug_info(
  keyboard_input: Res<Input<KeyCode>>,
  entity_query: Query<Entity>,
  snake_query: Query<&Transform, With<Snake>>,
  food_query: Query<Entity, With<Food>>,
) {
  if keyboard_input.just_pressed(KeyCode::P) {
    let debug = [
      "=== === === DEBUG === === ===",
      &format!("Entity Count: {}", entity_query.iter().count()),
      &format!(
        "Snakes: {:#?}",
        snake_query
          .iter()
          .map(|t| t.translation)
          .collect::<Vec<_>>()
      ),
      &format!("Food: {:#?}", food_query.get_single()),
    ]
    .join("\n");
    println!("{debug}");
  }
}
