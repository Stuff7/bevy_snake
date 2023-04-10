use super::{
  components::Food,
  events::{FoodEaten, SpawnFood},
};
use crate::{
  board::{
    components::Board,
    resources::GameBoard,
    utils::{create_cell_bundle, get_board_position},
  },
  snake::{
    components::{Living, Nourished, Snake, Speed},
    events::{BodySizeChange, SnakeSizeChange},
    MAX_SERPENTINE_DURATION, MIN_SERPENTINE_DURATION,
  },
};
use bevy::prelude::{
  BuildChildren, Commands, Entity, EventReader, EventWriter, Query, Res, Transform, With,
};
use rand::random;
use std::time::Duration;

pub(super) fn startup(mut spawn_food_writer: EventWriter<SpawnFood>) {
  spawn_food_writer.send(SpawnFood(Food::Regular));
  spawn_food_writer.send(SpawnFood(Food::ExtraGrowth));
  spawn_food_writer.send(SpawnFood(Food::Swiftness));
}

pub(super) fn spawn(
  mut commands: Commands,
  mut spawn_food_reader: EventReader<SpawnFood>,
  q_board: Query<Entity, With<Board>>,
  game_board: Res<GameBoard>,
) {
  for SpawnFood(food) in &mut spawn_food_reader {
    let Ok(board) = q_board.get_single() else {continue};
    let food = commands
      .spawn((
        *food,
        create_cell_bundle(
          (*food).into(),
          (random::<f32>() - 0.5) * game_board.width,
          (random::<f32>() - 0.5) * game_board.height,
        ),
      ))
      .id();
    commands.entity(board).add_child(food);
  }
}

pub(super) fn reposition(
  mut food_eaten_reader: EventReader<FoodEaten>,
  mut q_food: Query<&mut Transform>,
  game_board: Res<GameBoard>,
) {
  for eaten in food_eaten_reader.iter() {
    let Ok(mut food) = q_food.get_mut(eaten.food) else {continue};
    food.translation = get_board_position(
      (random::<f32>() - 0.5) * game_board.width,
      (random::<f32>() - 0.5) * game_board.height,
    );
  }
}

pub(super) fn apply_effects(
  mut commands: Commands,
  mut body_size_change_writer: EventWriter<SnakeSizeChange>,
  mut food_eaten_reader: EventReader<FoodEaten>,
  mut q_effect: Query<&Food>,
  mut q_snake: Query<(&mut Speed, Option<&mut Nourished>), (With<Snake>, With<Living>)>,
) {
  for FoodEaten { snake, food } in food_eaten_reader.iter() {
    let Ok(effect) = q_effect.get_mut(*food) else {continue};
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
