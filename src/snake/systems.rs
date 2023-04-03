use super::{
  components::{Direction, DirectionQueue, Player, SnakeBody, SnakeHead, SnakeSegment},
  events::{BodySizeChange, Serpentine},
  INITIAL_TAIL_LENGTH,
};
use crate::{
  food::{components::Food, events::FoodEaten},
  world::{CELL_HEIGHT, CELL_WIDTH},
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
  let head = SnakeHead {
    x: window.width() / 2.,
    y: window.height() / 2.,
  };
  let body = SnakeBody::new(&mut commands, head, INITIAL_TAIL_LENGTH);

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
  mut serpentine_writer: EventWriter<Serpentine>,
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
  serpentine_writer.send(Serpentine(Vec3::new(player_head.x, player_head.y, 0.)));
}

pub(super) fn snake_serpentining(
  mut serpentine_reader: EventReader<Serpentine>,
  mut head_query: Query<&mut Transform, With<SnakeHead>>,
) {
  for Serpentine(head) in serpentine_reader.iter().copied() {
    for mut transform in head_query.iter_mut() {
      transform.translation.x = head.x;
      transform.translation.y = head.y;
    }
  }
}

pub(super) fn snake_resizing(
  mut commands: Commands,
  mut size_change_reader: EventReader<BodySizeChange>,
  mut snake_query: Query<(&mut SnakeBody, &DirectionQueue), With<Player>>,
  snake_segment_query: Query<&Transform, With<SnakeSegment>>,
) {
  let Ok((mut body, direction)) = snake_query.get_single_mut() else {
    return;
  };
  use BodySizeChange::*;
  for event in size_change_reader.iter() {
    match event {
      Grow(size) => {
        if *size > 0 {
          let Ok(tail) = snake_segment_query.get(body.tail()) else { return; };
          let tail = tail.translation;
          let direction = direction.current.opposite();
          let tail_segments = (1..=*size).map(|i| {
            let w = CELL_WIDTH * i as f32;
            let h = CELL_HEIGHT * i as f32;
            let (x, y) = match direction {
              Direction::Bottom => (tail.x, tail.y - h),
              Direction::Right => (tail.x + w, tail.y),
              Direction::Top => (tail.x, tail.y + h),
              Direction::Left => (tail.x - w, tail.y),
            };
            SnakeSegment::spawn(&mut commands, x, y)
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

pub(super) fn snake_eating(
  mut serpentine_reader: EventReader<Serpentine>,
  mut food_eaten_writer: EventWriter<FoodEaten>,
  food_query: Query<&Transform, With<Food>>,
) {
  for Serpentine(head) in serpentine_reader.iter().copied() {
    let Ok(food_transform) = food_query.get_single() else { return; };

    let distance = food_transform.translation.distance(head);

    if distance < CELL_WIDTH {
      food_eaten_writer.send(FoodEaten);
    }
  }
}

pub(super) fn snake_dying(
  mut serpentine_reader: EventReader<Serpentine>,
  snake_segment_query: Query<&Transform, (With<SnakeSegment>, Without<SnakeHead>)>,
) {
  for Serpentine(head) in serpentine_reader.iter().copied() {
    if snake_segment_query
      .iter()
      .any(|segment| segment.translation == head)
    {
      println!("GAME OVER!");
      return;
    }
  }
}
