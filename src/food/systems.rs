use super::{components::Food, events::FoodEaten, FOOD_COLOR};
use crate::{
  board::{
    components::Board,
    utils::{create_cell_bundle, get_board_position},
    BOARD_SIZE,
  },
  player::events::RespawnPlayer,
  snake::events::{BodySizeChange, SnakeSizeChange},
};
use bevy::prelude::{
  BuildChildren, Commands, Entity, EventReader, EventWriter, Query, Transform, With,
};
use rand::random;

pub(super) fn spawn(
  mut commands: Commands,
  mut respawn_player_reader: EventReader<RespawnPlayer>,
  q_board: Query<Entity, With<Board>>,
) {
  for _ in respawn_player_reader.iter() {
    let Ok(board) = q_board.get_single() else {return};
    let food = commands
      .spawn((
        Food,
        create_cell_bundle(
          FOOD_COLOR,
          random::<f32>() * BOARD_SIZE - BOARD_SIZE / 2.,
          random::<f32>() * BOARD_SIZE - BOARD_SIZE / 2.,
        ),
      ))
      .id();
    commands.entity(board).add_child(food);
  }
}

pub(super) fn reposition(
  mut body_size_change_writer: EventWriter<SnakeSizeChange>,
  mut food_eaten_reader: EventReader<FoodEaten>,
  mut q_food: Query<&mut Transform, With<Food>>,
) {
  for FoodEaten { snake, food } in food_eaten_reader.iter() {
    let Ok(mut food) = q_food.get_mut(*food) else {continue};
    food.translation = get_board_position(
      random::<f32>() * BOARD_SIZE - BOARD_SIZE / 2.,
      random::<f32>() * BOARD_SIZE - BOARD_SIZE / 2.,
    );
    body_size_change_writer.send((*snake, BodySizeChange::Grow(1)));
  }
}
