use std::time::Duration;

use super::{
  components::Food,
  events::{FoodEaten, SpawnFood},
};
use crate::{
  board::{
    components::Board,
    utils::{create_cell_bundle, get_board_position},
    BOARD_SIZE,
  },
  snake::{
    components::{Living, Nourished, Snake, Speed},
    events::{BodySizeChange, SnakeSizeChange},
    MAX_SERPENTINE_DURATION, MIN_SERPENTINE_DURATION,
  },
};
use bevy::prelude::{
  BuildChildren, Commands, Entity, EventReader, EventWriter, Query, Transform, With,
};
use rand::random;

pub(super) fn startup(mut spawn_food_writer: EventWriter<SpawnFood>) {
  spawn_food_writer.send(SpawnFood(Food::Regular));
  spawn_food_writer.send(SpawnFood(Food::ExtraGrowth));
  spawn_food_writer.send(SpawnFood(Food::Swiftness));
}

pub(super) fn spawn(
  mut commands: Commands,
  mut spawn_food_reader: EventReader<SpawnFood>,
  q_board: Query<Entity, With<Board>>,
) {
  for SpawnFood(food) in &mut spawn_food_reader {
    let Ok(board) = q_board.get_single() else {continue};
    let food = commands
      .spawn((
        *food,
        create_cell_bundle(
          (*food).into(),
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
  mut q_snake: Query<(&mut Speed, Option<&mut Nourished>), (With<Snake>, With<Living>)>,
) {
  for FoodEaten { snake, food } in food_eaten_reader.iter() {
    let Ok((effect, mut food)) = q_food.get_mut(*food) else {continue};
    food.translation = get_board_position(
      random::<f32>() * BOARD_SIZE - BOARD_SIZE / 2.,
      random::<f32>() * BOARD_SIZE - BOARD_SIZE / 2.,
    );
    match *effect {
      Food::Regular => body_size_change_writer.send((*snake, BodySizeChange::Grow)),
      Food::ExtraGrowth => {
        let Ok((mut speed, nourished)) = q_snake.get_mut(*snake) else {continue};
        let serpentine_duration = speed.duration();
        if serpentine_duration < MAX_SERPENTINE_DURATION {
          speed.set_duration(serpentine_duration + Duration::from_millis(5));
        }
        if let Some(mut nourished) = nourished {
          nourished.0 += 4;
        } else {
          commands.entity(*snake).insert(Nourished(4));
        }
      }
      Food::Swiftness => {
        let Ok((mut speed, _)) = q_snake.get_mut(*snake) else {continue};
        let serpentine_duration = speed.duration();
        if serpentine_duration > MIN_SERPENTINE_DURATION {
          speed.set_duration(serpentine_duration - Duration::from_millis(5));
        }
      }
    }
  }
}
