use super::{
  components::{Food, FoodBundle},
  events::{FoodEaten, SpawnFood},
};
use crate::{
  board::{components::Board, resources::GameBoard, utils::get_board_position},
  effects::components::{Frozen, Swiftness},
  snake::{
    components::{Living, Satiety, Snake},
    events::{BodyResize, SnakeResize},
  },
  tetris::components::Tetrified,
};
use bevy::prelude::{
  BuildChildren, Commands, Entity, EventReader, EventWriter, Query, Res, Transform, With,
};
use rand::random;

pub(super) fn startup(mut spawn_food_writer: EventWriter<SpawnFood>) {
  spawn_food_writer.send(SpawnFood(Food::Regular));
  spawn_food_writer.send(SpawnFood(Food::Beefy));
  spawn_food_writer.send(SpawnFood(Food::Energetic));
  spawn_food_writer.send(SpawnFood(Food::Frozen));
  // spawn_food_writer.send(SpawnFood(Food::Tetris));
}

pub(super) fn spawn(
  mut commands: Commands,
  mut spawn_food_reader: EventReader<SpawnFood>,
  q_board: Query<Entity, With<Board>>,
  game_board: Res<GameBoard>,
) {
  for SpawnFood(food) in &mut spawn_food_reader {
    let Ok(board) = q_board.get_single() else {continue};
    let food = commands.spawn(FoodBundle::new(*food, &game_board)).id();
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
  mut body_size_change_writer: EventWriter<SnakeResize>,
  mut food_eaten_reader: EventReader<FoodEaten>,
  mut q_effect: Query<&Food>,
  mut q_snake: Query<
    (
      Entity,
      Option<&mut Satiety>,
      Option<&mut Swiftness>,
      Option<&mut Frozen>,
    ),
    (With<Snake>, With<Living>),
  >,
) {
  for FoodEaten { snake, food } in food_eaten_reader.iter() {
    let Ok(effect) = q_effect.get_mut(*food) else {continue};
    match *effect {
      Food::Regular => body_size_change_writer.send((*snake, BodyResize::Grow(1))),
      Food::Beefy => {
        let Ok((snake, mut satiety, _, _)) = q_snake.get_mut(*snake) else {continue};
        if let Some(ref mut level) = satiety {
          if level.0 < 3 {
            level.0 += 1;
          }
        } else {
          commands.entity(snake).insert(Satiety(1));
        }
        body_size_change_writer.send((snake, BodyResize::Grow(2)));
      }
      Food::Energetic => {
        let Ok((snake, _, mut swiftness, _)) = q_snake.get_mut(*snake) else {continue};
        if let Some(ref mut level) = swiftness {
          if level.0 > 2. {
            body_size_change_writer.send((snake, BodyResize::Grow(1)));
          } else {
            level.0 += 1.;
          }
        } else {
          commands.entity(snake).insert(Swiftness(1.));
        }
      }
      Food::Frozen => {
        for (other_snake, _, swiftness, mut frozen) in &mut q_snake {
          if *snake == other_snake || swiftness.map(|s| s.0 == 3.).unwrap_or_default() {
            continue;
          }
          if let Some(ref mut frozen) = frozen {
            frozen.increase_level();
          } else {
            commands.entity(other_snake).insert(Frozen::new());
          }
        }
      }
      Food::Tetris => {
        let Ok((snake, ..)) = q_snake.get_mut(*snake) else {continue};
        commands.entity(snake).insert(Tetrified);
      }
    }
  }
}
