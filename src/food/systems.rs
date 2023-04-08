use super::{components::Food, events::FoodEaten, FOOD_COLOR};
use crate::{
  board::{
    utils::{create_cell_bundle, get_board_position},
    BOARD_SIZE,
  },
  snake::events::{BodySizeChange, SnakeSizeChange},
};
use bevy::prelude::{Commands, EventReader, EventWriter, Query, Transform, With};
use rand::random;

pub(super) fn spawn(mut commands: Commands) {
  commands.spawn((
    Food,
    create_cell_bundle(
      FOOD_COLOR,
      random::<f32>() * BOARD_SIZE - BOARD_SIZE / 2.,
      random::<f32>() * BOARD_SIZE - BOARD_SIZE / 2.,
    ),
  ));
}

pub(super) fn reposition(
  mut body_size_change_writer: EventWriter<SnakeSizeChange>,
  mut food_eaten_reader: EventReader<FoodEaten>,
  mut q_food: Query<&mut Transform, With<Food>>,
) {
  let Ok(mut food) = q_food.get_single_mut() else {
    return;
  };

  for FoodEaten(snake) in food_eaten_reader.iter() {
    food.translation = get_board_position(
      random::<f32>() * BOARD_SIZE - BOARD_SIZE / 2.,
      random::<f32>() * BOARD_SIZE - BOARD_SIZE / 2.,
    );
    body_size_change_writer.send((*snake, BodySizeChange::Grow(1)));
  }
}
