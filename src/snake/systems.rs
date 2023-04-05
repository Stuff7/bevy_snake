use super::{
  components::{Direction, SnakeBody, SnakeHead, SnakeSegment},
  events::{BodySizeChange, Serpentine, SnakeSizeChange},
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
  mut commands: Commands,
  mut serpentine_writer: EventWriter<Serpentine>,
  mut snake_query: Query<(Entity, &Direction, &mut SnakeBody)>,
  mut snake_segment_query: Query<&mut Transform, With<SnakeSegment>>,
) {
  for (snake, direction, mut body) in snake_query.iter_mut() {
    let head_entity = body.head();
    let Ok(old_head) = snake_segment_query.get_mut(head_entity) else { continue; };

    let (x, y) =
      (old_head.translation.x, old_head.translation.y).add(direction.xy(CELL_WIDTH, CELL_HEIGHT));

    let mut new_head = if let Some(tail_entity) = body.pop_tail() {
      commands.entity(head_entity).remove::<SnakeHead>();
      commands.entity(tail_entity).insert(SnakeHead);
      body.push_head(tail_entity);
      let Ok(head) = snake_segment_query.get_mut(tail_entity) else { continue; };
      head
    } else {
      old_head
    };

    new_head.translation.x = x;
    new_head.translation.y = y;
    serpentine_writer.send(Serpentine(snake, new_head.translation));
  }
}

pub(super) fn resize(
  mut commands: Commands,
  mut size_change_reader: EventReader<SnakeSizeChange>,
  mut snake_query: Query<(&mut SnakeBody, &Direction)>,
  snake_segment_query: Query<(&Transform, &Sprite), With<SnakeSegment>>,
) {
  use BodySizeChange::*;
  for (snake, size_change) in size_change_reader.iter() {
    let Ok((mut body, direction)) = snake_query.get_mut(*snake) else {
      return;
    };
    match size_change {
      Grow(size) => {
        if *size > 0 {
          let Ok((tail, sprite)) = snake_segment_query.get(body.tail()) else { return; };
          let tail = tail.translation;
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
  mut serpentine_reader: EventReader<Serpentine>,
  snake_segment_query: Query<&Transform, (With<SnakeSegment>, Without<SnakeHead>)>,
) {
  for event in serpentine_reader.iter() {
    let head = event.1;
    if snake_segment_query
      .iter()
      .any(|segment| segment.translation == head)
    {
      println!("GAME OVER!");
      return;
    }
  }
}
