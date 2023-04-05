use crate::{
  food::components::Food,
  player::components::Player,
  snake::{
    components::SnakeBody,
    events::{BodySizeChange, SnakeSizeChange},
  },
};
use bevy::prelude::{Entity, EventWriter, Input, KeyCode, Query, Res, With};

pub(super) fn god_mode(
  mut size_change_event_writer: EventWriter<SnakeSizeChange>,
  player_query: Query<Entity, With<Player>>,
  keyboard_input: Res<Input<KeyCode>>,
) {
  use BodySizeChange::*;
  if keyboard_input.just_pressed(KeyCode::E) {
    let Ok(player) = player_query.get_single() else { return; };
    size_change_event_writer.send((player, Grow(1)));
  } else if keyboard_input.just_pressed(KeyCode::Q) {
    let Ok(player) = player_query.get_single() else { return; };
    size_change_event_writer.send((player, Shrink(1)));
  }
}

pub(super) fn print_debug_info(
  keyboard_input: Res<Input<KeyCode>>,
  entity_query: Query<Entity>,
  player_query: Query<&SnakeBody, With<Player>>,
  food_query: Query<Entity, With<Food>>,
) {
  if keyboard_input.just_pressed(KeyCode::P) {
    let debug = [
      "=== === === DEBUG === === ===",
      &format!("Entity Count: {}", entity_query.iter().count()),
      &format!("Player: {:#?}", player_query.get_single()),
      &format!("Food: {:#?}", food_query.get_single()),
    ]
    .join("\n");
    println!("{debug}");
  }
}
