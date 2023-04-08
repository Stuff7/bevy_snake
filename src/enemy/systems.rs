use super::{components::Enemy, INITIAL_ENEMY_LENGTH};
use crate::{
  board::{components::Board, BOARD_SIZE, CELL_SIZE},
  collections::TupleOps,
  color::generate_bright_color,
  food::components::Food,
  snake::{
    components::{snake_crashed, Direction, Snake, SnakeBundle, SnakeSegment},
    events::{Serpentine, SnakeDeath},
  },
};
use bevy::prelude::{
  BuildChildren, Commands, Entity, EventReader, EventWriter, Query, Transform, Vec3, With, Without,
};
use rand::random;

pub(super) fn startup(mut snake_death_writer: EventWriter<SnakeDeath>) {
  snake_death_writer.send(SnakeDeath);
}

pub(super) fn respawn(
  mut commands: Commands,
  mut snake_death_reader: EventReader<SnakeDeath>,
  q_board: Query<Entity, With<Board>>,
) {
  for _ in snake_death_reader.iter() {
    let Ok(board) = q_board.get_single() else {return};
    let enemy = (
      Enemy,
      SnakeBundle::new(
        &mut commands,
        board,
        random::<f32>() * BOARD_SIZE - BOARD_SIZE / 2.,
        random::<f32>() * BOARD_SIZE - BOARD_SIZE / 2.,
        generate_bright_color(),
        Direction::default(),
        INITIAL_ENEMY_LENGTH,
      ),
    );
    let enemy = commands.spawn(enemy).id();
    commands.entity(board).add_child(enemy);
  }
}

pub(super) fn seek_food(
  mut serpentine_reader: EventReader<Serpentine>,
  mut q_enemy: Query<&mut Direction, With<Enemy>>,
  q_food: Query<&Transform, With<Food>>,
  q_snake_head: Query<(Entity, &Transform), (With<Snake>, Without<SnakeSegment>)>,
  q_snake_segment: Query<&Transform, With<SnakeSegment>>,
) {
  let Ok(food) = q_food.get_single() else { return; };
  let food = food.translation;
  for Serpentine(enemy_entity, head) in serpentine_reader.iter().copied() {
    let Ok(mut direction) = q_enemy.get_mut(enemy_entity) else { continue; };
    for nearest in sort_direction_by_nearest(head, food) {
      if nearest == direction.opposite() {
        continue;
      }
      let (x, y) = (head.x, head.y).add(nearest.xy(CELL_SIZE + 4., CELL_SIZE + 4.));
      let head = Vec3::new(x, y, 0.);
      if !snake_crashed(
        q_snake_head.iter().map(|h| (h.0, h.1.translation)),
        q_snake_segment.iter().map(|h| h.translation),
        enemy_entity,
        head,
      ) {
        *direction = nearest;
        break;
      }
    }
  }
}

pub fn sort_direction_by_nearest(position: Vec3, target: Vec3) -> [Direction; 4] {
  use Direction::*;
  let direction_h = if position.x > target.x { Left } else { Right };
  let (x, y) = (position.x, position.y).add(direction_h.xy(CELL_SIZE, CELL_SIZE));
  let distance_h = target.distance(Vec3::new(x, y, 0.));

  let direction_v = if position.y > target.y { Bottom } else { Top };
  let (x, y) = (position.x, position.y).add(direction_v.xy(CELL_SIZE, CELL_SIZE));
  let distance_v = target.distance(Vec3::new(x, y, 0.));

  if distance_h < distance_v {
    [
      direction_h,
      direction_v,
      direction_v.opposite(),
      direction_h.opposite(),
    ]
  } else {
    [
      direction_v,
      direction_h,
      direction_h.opposite(),
      direction_v.opposite(),
    ]
  }
}
