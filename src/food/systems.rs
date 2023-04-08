use std::time::Duration;

use super::{components::Food, events::FoodEaten};
use crate::{
  board::{
    components::Board,
    utils::{create_cell_bundle, get_board_position},
    BOARD_SIZE,
  },
  player::events::RespawnPlayer,
  snake::{
    components::{Living, Nourished, Snake, Speed},
    events::{BodySizeChange, SnakeSizeChange},
  },
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
    let food = random::<Food>();
    let food = commands
      .spawn((
        food,
        create_cell_bundle(
          food.into(),
          random::<f32>() * BOARD_SIZE - BOARD_SIZE / 2.,
          random::<f32>() * BOARD_SIZE - BOARD_SIZE / 2.,
        ),
      ))
      .id();
    commands.entity(board).add_child(food);
  }
}

pub(super) fn reposition(
  mut commands: Commands,
  mut body_size_change_writer: EventWriter<SnakeSizeChange>,
  mut food_eaten_reader: EventReader<FoodEaten>,
  mut q_food: Query<(&Food, &mut Transform)>,
  mut q_snake: Query<&mut Speed, (With<Snake>, With<Living>)>,
) {
  for FoodEaten { snake, food } in food_eaten_reader.iter() {
    let Ok((effect, mut food)) = q_food.get_mut(*food) else {continue};
    food.translation = get_board_position(
      random::<f32>() * BOARD_SIZE - BOARD_SIZE / 2.,
      random::<f32>() * BOARD_SIZE - BOARD_SIZE / 2.,
    );
    match *effect {
      Food::None => body_size_change_writer.send((*snake, BodySizeChange::Grow)),
      Food::ExtraGrowth => {
        commands.entity(*snake).insert(Nourished(4));
      }
      Food::Swiftness => {
        let Ok(mut speed) = q_snake.get_mut(*snake) else {continue};
        let serpentine_duration = speed.duration();
        if serpentine_duration > Duration::from_millis(30) {
          speed.set_duration(serpentine_duration - Duration::from_millis(5));
        }
      }
    }
  }
}
