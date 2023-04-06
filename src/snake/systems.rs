use super::{
  components::{snake_crashed, Direction, Living, Snake, SnakeBody, SnakeSegment},
  events::{BodySizeChange, Serpentine, SnakeDeath, SnakeSizeChange},
};
use crate::{
  collections::TupleOps,
  food::{components::Food, events::FoodEaten},
  world::{CELL_HEIGHT, CELL_WIDTH},
};
use bevy::prelude::{
  Commands, Entity, EventReader, EventWriter, Query, Sprite, Transform, With, Without,
};

pub(super) fn serpentine(
  mut serpentine_writer: EventWriter<Serpentine>,
  mut snake_query: Query<
    (Entity, &mut Transform, &Direction, &mut SnakeBody),
    (With<Snake>, With<Living>),
  >,
  mut snake_segment_query: Query<&mut Transform, (With<SnakeSegment>, Without<Snake>)>,
) {
  for (snake, mut snake_head, direction, mut body) in snake_query.iter_mut() {
    if let Some(head_entity) = body.head() {
      let tail = if let Some(tail_entity) = body.pop_tail() {
        body.push_head(tail_entity);
        tail_entity
      } else {
        head_entity
      };
      let Ok(mut old_head) = snake_segment_query.get_mut(tail) else { continue; };
      old_head.translation = snake_head.translation;
    }

    let (x, y) = direction.xy(CELL_WIDTH, CELL_HEIGHT);
    snake_head.translation.x += x;
    snake_head.translation.y += y;

    serpentine_writer.send(Serpentine(snake, snake_head.translation));
  }
}

pub(super) fn resize(
  mut commands: Commands,
  mut size_change_reader: EventReader<SnakeSizeChange>,
  mut snake_query: Query<(&mut SnakeBody, &Transform, &Direction, &Sprite), With<Snake>>,
  snake_segment_query: Query<&Transform, With<SnakeSegment>>,
) {
  use BodySizeChange::*;
  for (snake, size_change) in size_change_reader.iter() {
    let Ok((mut body, head, direction, sprite)) = snake_query.get_mut(*snake) else {
      return;
    };
    match size_change {
      Grow(size) => {
        if *size > 0 {
          let tail = if let Some(tail) = body.tail() {
            let Ok(tail) = snake_segment_query.get(tail) else { return; };
            tail
          } else {
            head
          }
          .translation;
          let direction = direction.opposite();
          let tail_segments = (1..=*size).map(|i| {
            let (x, y) =
              (tail.x, tail.y).add(direction.xy(CELL_WIDTH * i as f32, CELL_HEIGHT * i as f32));
            SnakeSegment::spawn(&mut commands, sprite.color, x, y)
          });
          body.extend_tail(tail_segments);
        }
      }
      Shrink(size) => {
        for _ in 0..*size {
          let Some(tail) = body.pop_tail() else { return; };
          commands.entity(tail).despawn();
        }
      }
    }
  }
}

pub(super) fn eat(
  mut serpentine_reader: EventReader<Serpentine>,
  mut food_eaten_writer: EventWriter<FoodEaten>,
  food_query: Query<&Transform, With<Food>>,
) {
  for Serpentine(snake, head) in serpentine_reader.iter().copied() {
    let Ok(food_transform) = food_query.get_single() else { return; };
    let distance = food_transform.translation.distance(head);

    if distance < CELL_WIDTH {
      food_eaten_writer.send(FoodEaten(snake));
    }
  }
}

pub(super) fn die(
  mut commands: Commands,
  mut serpentine_reader: EventReader<Serpentine>,
  snake_head_query: Query<(Entity, &Transform), (With<Snake>, Without<SnakeSegment>)>,
  snake_segment_query: Query<&Transform, (With<SnakeSegment>, Without<Snake>)>,
) {
  for Serpentine(snake_entity, snake_head) in serpentine_reader.iter().copied() {
    if snake_crashed(
      snake_head_query.iter().map(|h| (h.0, h.1.translation)),
      snake_segment_query.iter().map(|h| h.translation),
      snake_entity,
      snake_head,
    ) {
      println!("Snake {snake_entity:?} DIED");
      commands.entity(snake_entity).remove::<Living>();
      return;
    }
  }
}

pub(super) fn despawn(
  mut commands: Commands,
  mut snake_death_writer: EventWriter<SnakeDeath>,
  mut snake_query: Query<(Entity, &mut SnakeBody), (With<Snake>, Without<Living>)>,
) {
  for (snake, mut body) in snake_query.iter_mut() {
    commands
      .entity(body.pop_tail().unwrap_or_else(|| {
        snake_death_writer.send(SnakeDeath);
        snake
      }))
      .despawn();
  }
}
