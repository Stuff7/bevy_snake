use super::{
  components::{Direction, Living, Snake, SnakeBody, SnakeSegment, Speed},
  events::{BodySizeChange, Serpentine, SnakeDeath, SnakeSizeChange},
  utils::snake_crashed,
};
use crate::{
  board::{components::Board, BOARD_SIZE, CELL_SIZE, HALF_CELL_SIZE},
  collections::TupleOps,
  food::{components::Food, events::FoodEaten},
};
use bevy::prelude::{
  BuildChildren, Commands, Entity, EventReader, EventWriter, Query, Res, Sprite, Time, Transform,
  With, Without,
};

pub(super) fn serpentine(
  mut serpentine_writer: EventWriter<Serpentine>,
  mut q_snake: Query<
    (
      Entity,
      &mut Transform,
      &Direction,
      &mut SnakeBody,
      &mut Speed,
    ),
    (With<Snake>, With<Living>),
  >,
  mut q_snake_segment: Query<&mut Transform, (With<SnakeSegment>, Without<Snake>)>,
  time: Res<Time>,
) {
  for (snake, mut snake_head, direction, mut body, mut speed) in &mut q_snake {
    speed.tick(time.delta());
    if !speed.finished() {
      continue;
    }
    if let Some(head_entity) = body.head() {
      let tail = if let Some(tail_entity) = body.pop_tail() {
        body.push_head(tail_entity);
        tail_entity
      } else {
        head_entity
      };
      let Ok(mut old_head) = q_snake_segment.get_mut(tail) else { continue; };
      old_head.translation = snake_head.translation;
    }

    let (x, y) = direction.xy(CELL_SIZE, CELL_SIZE);
    snake_head.translation.x += x;
    snake_head.translation.y += y;

    let width = BOARD_SIZE;
    let height = BOARD_SIZE;

    if snake_head.translation.x >= width / 2. {
      snake_head.translation.x = HALF_CELL_SIZE - width / 2.;
    } else if snake_head.translation.x < -width / 2. {
      snake_head.translation.x = width / 2. - HALF_CELL_SIZE;
    }
    if snake_head.translation.y >= height / 2. {
      snake_head.translation.y = HALF_CELL_SIZE - height / 2.;
    } else if snake_head.translation.y < -height / 2. {
      snake_head.translation.y = height / 2. - HALF_CELL_SIZE;
    }

    serpentine_writer.send(Serpentine(snake, snake_head.translation));
  }
}

pub(super) fn resize(
  mut commands: Commands,
  mut size_change_reader: EventReader<SnakeSizeChange>,
  mut q_snake: Query<
    (&mut SnakeBody, &Transform, &Direction, &Sprite),
    (With<Snake>, With<Living>),
  >,
  q_snake_segment: Query<&Transform, With<SnakeSegment>>,
  q_board: Query<Entity, With<Board>>,
) {
  use BodySizeChange::*;
  for (snake, size_change) in &mut size_change_reader {
    let Ok(board) = q_board.get_single() else {return};
    let Ok((mut body, head, direction, sprite)) = q_snake.get_mut(*snake) else {
      return;
    };
    match size_change {
      Grow(size) => {
        if *size > 0 {
          let tail = if let Some(tail) = body.tail() {
            let Ok(tail) = q_snake_segment.get(tail) else { return; };
            tail
          } else {
            head
          }
          .translation;
          let direction = direction.opposite();
          let tail_segments = (1..=*size).map(|i| {
            let (x, y) =
              (tail.x, tail.y).add(direction.xy(CELL_SIZE * i as f32, CELL_SIZE * i as f32));
            SnakeSegment::spawn(&mut commands, board, sprite.color, x, y)
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
  q_food: Query<&Transform, With<Food>>,
) {
  for Serpentine(snake, head) in serpentine_reader.iter().copied() {
    let Ok(food_transform) = q_food.get_single() else { return; };
    let distance = food_transform.translation.distance(head);

    if distance < CELL_SIZE {
      food_eaten_writer.send(FoodEaten(snake));
    }
  }
}

pub(super) fn die(
  mut commands: Commands,
  mut serpentine_reader: EventReader<Serpentine>,
  q_snake_head: Query<(Entity, &Transform), (With<Snake>, With<Living>)>,
  q_snake_segment: Query<&Transform, With<SnakeSegment>>,
) {
  for Serpentine(snake_entity, snake_head) in serpentine_reader.iter().copied() {
    if snake_crashed(
      q_snake_head.iter().map(|h| (h.0, h.1.translation)),
      q_snake_segment.iter().map(|h| h.translation),
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
  mut q_snake: Query<(Entity, &mut SnakeBody), (With<Snake>, Without<Living>)>,
  q_board: Query<Entity, With<Board>>,
) {
  for (snake, mut body) in &mut q_snake {
    let Ok(board) = q_board.get_single() else {return};
    let tail = body.pop_tail().unwrap_or_else(|| {
      snake_death_writer.send(SnakeDeath);
      snake
    });
    commands.entity(board).remove_children(&[tail]);
    commands.entity(tail).despawn();
  }
}
