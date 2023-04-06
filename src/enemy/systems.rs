use super::{components::Enemy, INITIAL_ENEMY_LENGTH};
use crate::{
  collections::TupleOps,
  food::components::Food,
  snake::{
    components::{snake_crashed, Direction, Snake, SnakeBundle, SnakeSegment},
    events::{Serpentine, SnakeDeath},
  },
  world::{CELL_HEIGHT, CELL_WIDTH},
};
use bevy::{
  prelude::{Color, Commands, Entity, EventReader, Query, Transform, Vec3, With, Without},
  window::{PrimaryWindow, Window},
};
use rand::random;

pub(super) fn spawn(mut commands: Commands, window: Query<&Window, With<PrimaryWindow>>) {
  spawn_enemy(&mut commands, window.get_single().unwrap());
}

pub(super) fn respawn(
  mut commands: Commands,
  mut snake_death_reader: EventReader<SnakeDeath>,
  window: Query<&Window, With<PrimaryWindow>>,
) {
  for _ in snake_death_reader.iter() {
    spawn_enemy(&mut commands, window.get_single().unwrap());
  }
}

pub(super) fn seek_food(
  mut serpentine_reader: EventReader<Serpentine>,
  mut enemy_query: Query<(Entity, &mut Direction, &Transform), With<Enemy>>,
  food_query: Query<&Transform, With<Food>>,
  snake_head_query: Query<(Entity, &Transform), (With<Snake>, Without<SnakeSegment>)>,
  snake_segment_query: Query<&Transform, With<SnakeSegment>>,
) {
  let Ok(food) = food_query.get_single() else { return; };
  let food = food.translation;
  for _ in serpentine_reader.iter() {
    for (enemy_entity, mut direction, head) in enemy_query.iter_mut() {
      let head = head.translation;
      for nearest in sort_direction_by_nearest(head, food) {
        if nearest == direction.opposite() {
          continue;
        }
        let (x, y) = (head.x, head.y).add(nearest.xy(CELL_WIDTH + 4., CELL_HEIGHT + 4.));
        let head = Vec3::new(x, y, 0.);
        if !snake_crashed(
          snake_head_query.iter().map(|h| (h.0, h.1.translation)),
          snake_segment_query.iter().map(|h| h.translation),
          enemy_entity,
          head,
        ) {
          *direction = nearest;
          break;
        }
      }
    }
  }
}

pub fn sort_direction_by_nearest(position: Vec3, target: Vec3) -> [Direction; 4] {
  use Direction::*;
  let direction_h = if position.x > target.x { Left } else { Right };
  let (x, y) = (position.x, position.y).add(direction_h.xy(CELL_WIDTH, CELL_HEIGHT));
  let distance_h = target.distance(Vec3::new(x, y, 0.));

  let direction_v = if position.y > target.y { Bottom } else { Top };
  let (x, y) = (position.x, position.y).add(direction_v.xy(CELL_WIDTH, CELL_HEIGHT));
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

fn spawn_enemy(commands: &mut Commands, window: &Window) {
  let enemy = (
    Enemy,
    SnakeBundle::new(
      commands,
      random::<f32>() * window.width(),
      random::<f32>() * window.height(),
      Color::rgb(
        random::<f32>() / 2. + 0.5,
        random::<f32>() / 2. + 0.5,
        random::<f32>() / 2. + 0.5,
      ),
      Direction::default(),
      INITIAL_ENEMY_LENGTH,
    ),
  );

  commands.spawn(enemy);
}
