use crate::{
  food::{components::Food, events::FoodEaten},
  world::{CELL_HEIGHT, CELL_WIDTH},
};

use super::{
  components::{Direction, DirectionQueue, Player, SnakeBody, SnakeHead},
  events::BodySizeChange,
};
use bevy::{
  prelude::{
    Commands, EventReader, EventWriter, Input, KeyCode, Query, Res, Transform, Vec3, With,
    Without,
  },
  window::{PrimaryWindow, Window},
};

pub(super) fn snake_spawning(
  mut commands: Commands,
  window: Query<&Window, With<PrimaryWindow>>,
) {
  let window = window.get_single().unwrap();
  let (head_entity, head) =
    SnakeHead::spawn_and_create(&mut commands, window.width() / 2., window.height() / 2.);
  let body = SnakeBody::new(head_entity);

  commands.spawn((Player, DirectionQueue::default(), body, head));
}

pub(super) fn snake_steering(
  keyboard_input: Res<Input<KeyCode>>,
  mut player_query: Query<&mut DirectionQueue, With<Player>>,
) {
  let Ok(mut direction) = player_query.get_single_mut() else { return; };
  if let Some(new_direction) = direction.current.next_from_input(&keyboard_input) {
    if direction.current == direction.previous {
      direction.current = new_direction;
    } else {
      direction.next = Some(new_direction);
    }
  };
}

pub(super) fn snake_head_positioning(
  mut commands: Commands,
  mut snake_query: Query<
    (&mut DirectionQueue, &mut SnakeBody, &mut SnakeHead),
    With<Player>,
  >,
) {
  let Ok((mut direction, mut body, mut player_head)) = snake_query.get_single_mut() else {
    return;
  };

  let old_head = body.head();

  direction.advance();

  match direction.current {
    Direction::Bottom => player_head.y -= CELL_HEIGHT,
    Direction::Right => player_head.x += CELL_WIDTH,
    Direction::Top => player_head.y += CELL_HEIGHT,
    Direction::Left => player_head.x -= CELL_WIDTH,
  };

  if let Some(tail) = body.pop_tail() {
    commands.entity(tail).insert(*player_head);
    body.push_head(tail);
    commands.entity(old_head).remove::<SnakeHead>();
  } else {
    commands.entity(old_head).insert(*player_head);
  }
}

pub(super) fn snake_serpentining(
  mut head_query: Query<(&SnakeHead, &mut Transform), Without<Player>>,
) {
  for (head, mut transform) in head_query.iter_mut() {
    transform.translation.x = head.x;
    transform.translation.y = head.y;
  }
}

pub(super) fn snake_resizing(
  mut commands: Commands,
  mut size_change_reader: EventReader<BodySizeChange>,
  mut snake_query: Query<(&mut SnakeBody, &SnakeHead), With<Player>>,
) {
  let Ok((mut body, head)) = snake_query.get_single_mut() else {
    return;
  };
  use BodySizeChange::*;
  for event in size_change_reader.iter() {
    match event {
      Grow(size) => {
        if *size > 0 {
          let x = head.x;
          let y = head.y;
          commands.entity(body.head()).remove::<SnakeHead>();
          for _ in 0..*size {
            body.push_head(SnakeHead::spawn(&mut commands, x, y));
          }
          commands.entity(body.head()).insert(*head);
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

pub(super) fn snake_eating(
  mut food_eaten_writer: EventWriter<FoodEaten>,
  snake_query: Query<&SnakeHead, With<Player>>,
  food_query: Query<&Transform, With<Food>>,
) {
  let Ok(head) = snake_query.get_single() else { return; };
  let Ok(food_transform) = food_query.get_single() else { return; };

  let distance = food_transform
    .translation
    .distance(Vec3::new(head.x, head.y, 0.));

  if distance < CELL_WIDTH {
    food_eaten_writer.send(FoodEaten);
  }
}
