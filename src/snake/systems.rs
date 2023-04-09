use super::{
  components::{Direction, Living, Nourished, Snake, SnakeBody, SnakeSegment, Speed},
  events::{BodySizeChange, Serpentine, SnakeDeath, SnakeSizeChange},
  utils::snake_crashed,
};
use crate::{
  board::{components::Board, BOARD_SIZE, CELL_SIZE, HALF_CELL_SIZE},
  collections::TupleOps,
  food::{components::Food, events::FoodEaten},
  scoreboard::components::Score,
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
    let Ok(board) = q_board.get_single() else {continue};
    let Ok((mut body, head, direction, sprite)) = q_snake.get_mut(*snake) else {
      continue;
    };
    match size_change {
      Grow => {
        let tail = if let Some(tail) = body.tail() {
          let Ok(tail) = q_snake_segment.get(tail) else {continue};
          tail
        } else {
          head
        }
        .translation;
        let direction = direction.opposite();
        let (x, y) = (tail.x, tail.y).add(direction.xy(CELL_SIZE, CELL_SIZE));
        let tail = SnakeSegment::spawn(&mut commands, board, sprite.color, x, y);
        body.push_tail(tail);
      }
      Shrink => {
        let Some(tail) = body.pop_tail() else { return; };
        commands.entity(tail).despawn();
      }
    }
  }
}

pub(super) fn grow(
  mut commands: Commands,
  mut serpentine_reader: EventReader<Serpentine>,
  mut q_snake: Query<
    (&Sprite, &Direction, &mut SnakeBody, &mut Nourished),
    (With<Snake>, With<Living>),
  >,
  q_snake_segment: Query<&Transform, With<SnakeSegment>>,
  q_board: Query<Entity, With<Board>>,
) {
  for snake in &mut serpentine_reader {
    let Ok(
      (sprite, direction, mut body, mut nourished_lvl)
    ) = q_snake.get_mut(snake.0) else {continue};

    if nourished_lvl.0 == 0 {
      commands.entity(snake.0).remove::<Nourished>();
      return;
    }

    let Ok(board) = q_board.get_single() else {continue};
    let Ok(tail) = q_snake_segment.get(body.tail().unwrap_or(snake.0)) else {continue};
    let tail = tail.translation;
    let direction = direction.opposite();
    let (x, y) = (tail.x, tail.y).add(direction.xy(CELL_SIZE, CELL_SIZE));
    let tail = SnakeSegment::spawn(&mut commands, board, sprite.color, x, y);
    body.push_tail(tail);
    nourished_lvl.0 -= 1;
  }
}

pub(super) fn eat(
  mut serpentine_reader: EventReader<Serpentine>,
  mut food_eaten_writer: EventWriter<FoodEaten>,
  q_food: Query<(Entity, &Transform), With<Food>>,
) {
  for Serpentine(snake, head) in serpentine_reader.iter().copied() {
    for (food, food_transform) in &q_food {
      if food_transform.translation.distance(head) < CELL_SIZE {
        food_eaten_writer.send(FoodEaten { snake, food });
      }
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

pub(super) fn update_score(
  mut serpentine_reader: EventReader<Serpentine>,
  mut q_snake: Query<(&SnakeBody, &mut Score), (With<Snake>, With<Living>)>,
) {
  for serpentine in &mut serpentine_reader {
    let Ok((body, mut score)) = q_snake.get_mut(serpentine.0) else {continue};
    score.0 = body.len();
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
