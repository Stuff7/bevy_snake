use crate::{
  food::components::Food,
  snake::{components::SnakeBody, events::BodySizeChange},
};
use bevy::prelude::{Entity, EventWriter, Input, KeyCode, Query, Res, With};

pub(super) fn god_mode(
  mut size_change_event_writer: EventWriter<BodySizeChange>,
  keyboard_input: Res<Input<KeyCode>>,
) {
  use BodySizeChange::*;
  if keyboard_input.just_pressed(KeyCode::E) {
    size_change_event_writer.send(Grow(1));
  } else if keyboard_input.just_pressed(KeyCode::Q) {
    size_change_event_writer.send(Shrink(1));
  }
}

pub(super) fn print_debug_info(
  keyboard_input: Res<Input<KeyCode>>,
  entity_query: Query<Entity>,
  player_query: Query<&SnakeBody>,
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
