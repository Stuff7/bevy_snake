use super::{
  components::{
    Direction, Hunger, Living, Nourishment, Revive, Satiety, Seeker, Snake, SnakeBody, SnakeBundle,
    SnakeSegment, SnakeSegmentBundle,
  },
  events::{BodyResize, Serpentine, SnakeResize},
  utils::{snake_crashed, sort_direction_by_nearest},
};
use crate::{
  attributes::components::{MoveCooldown, Speed},
  board::{
    components::Board, resources::GameBoard, utils::get_board_position, CELL_SIZE, HALF_CELL_SIZE,
  },
  collections::ExternalOps,
  effects::components::Swiftness,
  food::{components::Food, events::FoodEaten},
  scoreboard::components::{Name, Score, ScoreEntity},
  tetris::components::{Tetrified, TetrisBlockBundle},
};
use bevy::prelude::{
  Added, BuildChildren, Changed, Commands, Entity, EventReader, EventWriter, Query, Res, Sprite,
  Time, Transform, Vec3, Visibility, With, Without,
};
use rand::random;

pub(super) fn serpentine(
  mut serpentine_writer: EventWriter<Serpentine>,
  mut q_snake: Query<
    (
      Entity,
      &mut Transform,
      &Direction,
      &mut SnakeBody,
      &mut MoveCooldown,
    ),
    (With<Snake>, With<Living>),
  >,
  mut q_snake_segment: Query<&mut Transform, (With<SnakeSegment>, Without<Snake>)>,
  game_board: Res<GameBoard>,
  time: Res<Time>,
) {
  for (snake, mut snake_head, direction, mut body, mut move_cooldown) in &mut q_snake {
    if !move_cooldown.finished(time.delta()) {
      continue;
    }
    if let Some(head_entity) = body.head() {
      let tail = if let Some(tail_entity) = body.pop_tail() {
        body.push_head(tail_entity);
        tail_entity
      } else {
        head_entity
      };
      let Ok(mut old_head_transform) = q_snake_segment.get_mut(tail) else { continue; };
      old_head_transform.translation = snake_head.translation;
    }

    let (x, y) = direction.xy(CELL_SIZE, CELL_SIZE);
    snake_head.translation.x += x;
    snake_head.translation.y += y;

    let GameBoard { width, height } = *game_board;

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

pub(super) fn recolor(
  mut q_snake: Query<(&Sprite, &SnakeBody), (With<Snake>, Changed<Sprite>)>,
  mut q_snake_segment: Query<&mut Sprite, (With<SnakeSegment>, Without<Snake>)>,
) {
  for (head, body) in &mut q_snake {
    for segment in body.iter() {
      let Ok(mut segment) = q_snake_segment.get_mut(*segment) else {continue};
      segment.color = head.color;
    }
  }
}

pub(super) fn resize(
  mut commands: Commands,
  mut resize_reader: EventReader<SnakeResize>,
  mut q_snake: Query<
    (
      Option<&mut Nourishment>,
      Option<&mut Hunger>,
      Option<&Satiety>,
    ),
    (With<Living>, With<Snake>),
  >,
) {
  for (snake, resize) in &mut resize_reader {
    let Ok((mut nourishment, mut hunger, satiety)) = q_snake.get_mut(*snake) else {continue};
    match *resize {
      BodyResize::Grow(n) => {
        let satiety = satiety.map(|s| s.0).unwrap_or(1);
        if let Some(ref mut nourishment) = nourishment {
          nourishment.0 += n * satiety;
        } else {
          commands.entity(*snake).insert(Nourishment(n * satiety));
        }
      }
      BodyResize::Shrink(n) => {
        if let Some(ref mut hunger) = hunger {
          hunger.0 += n;
        } else {
          commands.entity(*snake).insert(Hunger(n));
        }
      }
    }
  }
}

pub(super) fn grow(
  mut commands: Commands,
  mut serpentine_reader: EventReader<Serpentine>,
  mut q_snake: Query<
    (
      &Transform,
      &Sprite,
      &Direction,
      &mut SnakeBody,
      &mut Nourishment,
    ),
    (With<Snake>, With<Living>),
  >,
  q_snake_segment: Query<&Transform, With<SnakeSegment>>,
) {
  for snake in &mut serpentine_reader {
    let Ok(
      (head, sprite, direction, mut body, mut nourishment)
    ) = q_snake.get_mut(snake.0) else {continue};

    if nourishment.0 == 0 {
      commands.entity(snake.0).remove::<Nourishment>();
      return;
    }

    let tail = q_snake_segment
      .get(body.tail().unwrap_or(snake.0))
      .unwrap_or(head);
    let tail = tail.translation;
    let direction = direction.opposite();
    let (x, y) = (tail.x, tail.y).add(direction.xy(CELL_SIZE, CELL_SIZE));
    let tail = commands
      .spawn(SnakeSegmentBundle::new(sprite.color, x, y))
      .id();
    body.push_tail(tail);
    nourishment.0 -= 1;
  }
}

pub(super) fn shrink(
  mut commands: Commands,
  mut serpentine_reader: EventReader<Serpentine>,
  mut q_snake: Query<(&mut SnakeBody, &mut Hunger), (With<Snake>, With<Living>)>,
) {
  for snake in &mut serpentine_reader {
    let Ok(
      (mut body, mut hunger)
    ) = q_snake.get_mut(snake.0) else {continue};
    let mut snake = commands.entity(snake.0);

    if hunger.0 == 0 {
      snake.remove::<Hunger>();
      continue;
    }

    let Some(tail) = body.pop_tail() else {
      snake.remove::<Hunger>().remove::<Living>();
      continue;
    };

    commands.entity(tail).despawn();
    hunger.0 -= 1;
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
      commands.entity(snake_entity).remove::<Living>();
      return;
    }
  }
}

pub(super) fn revive(
  mut commands: Commands,
  mut q_snake: Query<(Entity, &mut Visibility, &mut Transform), (Added<Revive>, With<Snake>)>,
  game_board: Res<GameBoard>,
) {
  for (snake, mut visibility, mut transform) in &mut q_snake {
    transform.translation = get_board_position(
      (random::<f32>() - 0.5) * game_board.width,
      (random::<f32>() - 0.5) * game_board.height,
    );
    *visibility = Visibility::Visible;
    commands
      .entity(snake)
      .remove::<Revive>()
      .insert(Living)
      .insert(Swiftness(0.))
      .insert(Nourishment(4));
  }
}

pub(super) fn disappear(
  mut commands: Commands,
  mut q_snakes: Query<
    (&ScoreEntity, &mut Visibility, &mut SnakeBody),
    (With<Snake>, Without<Living>),
  >,
  q_scores: Query<&Name, With<Score>>,
  q_board: Query<Entity, With<Board>>,
) {
  for (score, mut visibility, mut body) in &mut q_snakes {
    let Ok(board) = q_board.get_single() else {return};
    let Ok(name) = q_scores.get(score.0) else {return};
    if let Some(tail) = body.pop_tail() {
      commands.entity(board).remove_children(&[tail]);
      commands.entity(tail).despawn();
    } else if *visibility != Visibility::Hidden {
      println!("☠️ {}", name.0);
      *visibility = Visibility::Hidden;
    }
  }
}

pub(super) fn update_score(
  mut serpentine_reader: EventReader<Serpentine>,
  mut q_snakes: Query<(&SnakeBody, &ScoreEntity), With<Snake>>,
  mut q_scores: Query<&mut Score>,
) {
  for serpentine in &mut serpentine_reader {
    let Ok((body, score)) = q_snakes.get_mut(serpentine.0) else {continue};
    let Ok(mut score) = q_scores.get_mut(score.0) else {continue};
    score.0 = body.len();
  }
}

pub(super) fn seek(
  mut serpentine_reader: EventReader<Serpentine>,
  mut q_seeker: Query<(&Seeker, &mut Direction)>,
  q_snake_head: Query<(Entity, &Transform), (With<Snake>, Without<SnakeSegment>)>,
  q_snake_segment: Query<&Transform, With<SnakeSegment>>,
  game_board: Res<GameBoard>,
) {
  for Serpentine(enemy_entity, head) in serpentine_reader.iter().copied() {
    let Ok((seeker, mut direction)) = q_seeker.get_mut(enemy_entity) else { continue; };
    for nearest in sort_direction_by_nearest(head, seeker.0, &game_board) {
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

pub(super) fn tetrify(
  mut commands: Commands,
  mut q_snake: Query<
    (Entity, &Transform, &Sprite, &Speed, &SnakeBody),
    (With<Snake>, With<Living>, With<Tetrified>),
  >,
) {
  for (snake, head_transform, head_sprite, speed, body) in &mut q_snake {
    let (a, b) = body.as_slices();
    TetrisBlockBundle::insert_to(
      commands
        .entity(snake)
        .remove::<Tetrified>()
        .remove::<SnakeBundle>(),
      (*head_transform, head_sprite.clone()),
      speed.0,
      &[a, b].concat()[..],
    );
  }
}
