use super::{components::Enemy, events::SpawnEnemy, INITIAL_ENEMY_LENGTH};
use crate::{
  board::{components::Board, resources::GameBoard, CELL_SIZE},
  collections::TupleOps,
  color::generate_bright_color,
  food::components::Food,
  snake::{
    components::{Direction, Snake, SnakeBundle, SnakeConfig, SnakeSegment},
    events::Serpentine,
    utils::snake_crashed,
  },
};
use bevy::prelude::{
  BuildChildren, Commands, Entity, EventReader, EventWriter, Query, Res, Transform, Vec3, With,
  Without,
};
use rand::random;

pub(super) fn respawn(mut spawn_enemy_writer: EventWriter<SpawnEnemy>) {
  spawn_enemy_writer.send(SpawnEnemy);
}

pub(super) fn spawn(
  mut commands: Commands,
  mut spawn_enemy_reader: EventReader<SpawnEnemy>,
  q_board: Query<Entity, With<Board>>,
  game_board: Res<GameBoard>,
) {
  for _ in spawn_enemy_reader.iter() {
    let Ok(board) = q_board.get_single() else {return};
    let enemy = (
      Enemy,
      SnakeBundle::new(
        &mut commands,
        board,
        SnakeConfig {
          x: (random::<f32>() - 0.5) * game_board.width,
          y: (random::<f32>() - 0.5) * game_board.height,
          color: generate_bright_color(),
          tail_length: INITIAL_ENEMY_LENGTH,
          ..Default::default()
        },
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
  for Serpentine(enemy_entity, head) in serpentine_reader.iter().copied() {
    let Ok(mut direction) = q_enemy.get_mut(enemy_entity) else { continue; };
    let Some((_, food)) = q_food
      .iter()
      .map(|food| (food.translation.distance(head), food.translation))
      .min_by(|(a, _), (b, _)| a.partial_cmp(b).unwrap()) else {continue};
    for nearest in sort_direction_by_nearest(head, food) {
      if nearest == direction.opposite() {
        continue;
      }
      let (x, y) = (head.x, head.y).add(nearest.xy(CELL_SIZE, CELL_SIZE));
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
