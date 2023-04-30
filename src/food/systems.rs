use super::{
  components::{Food, FoodBundle},
  events::{FoodEaten, SpawnFood},
};
use crate::{
  board::{components::RandomCellPosition, resources::GameBoard},
  effects::components::{Frozen, Swiftness},
  snake::{
    components::{Living, Satiety, Snake},
    events::{BodyResize, SnakeResize},
  },
  tetris::components::Tetrified,
};
use bevy::prelude::{Commands, Entity, EventReader, EventWriter, Query, Res, With};

pub(super) fn startup(mut spawn_food_writer: EventWriter<SpawnFood>) {
  spawn_food_writer.send(SpawnFood(Food::Regular));
  spawn_food_writer.send(SpawnFood(Food::Beefy));
  spawn_food_writer.send(SpawnFood(Food::Energetic));
  spawn_food_writer.send(SpawnFood(Food::Frozen));
  spawn_food_writer.send(SpawnFood(Food::Tetris));
}

pub(super) fn spawn(
  mut commands: Commands,
  mut spawn_food_reader: EventReader<SpawnFood>,
  game_board: Res<GameBoard>,
) {
  for SpawnFood(food) in &mut spawn_food_reader {
    commands.spawn(FoodBundle::new(*food, &game_board));
  }
}

pub(super) fn consume(
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
  for FoodEaten { eater: snake, food } in food_eaten_reader.iter() {
    commands.entity(*food).insert(RandomCellPosition);
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
