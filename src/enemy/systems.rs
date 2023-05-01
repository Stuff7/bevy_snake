use super::{
  components::{Enemy, Glutton, Killer, Omnivorous, Speedster, TargetLocked},
  GLUTTON_COLOR, INITIAL_ENEMY_LENGTH, KILLER_COLOR, OMNIVOROUS_COLOR, SPEEDSTER_COLOR,
};
use crate::{
  attributes::components::MoveCooldown,
  board::{
    resources::GameBoard,
    utils::{iter_cells, iter_cells_from_to},
  },
  food::components::Food,
  snake::{
    components::{Living, Revive, Seeker, SnakeBundle, SnakeConfig},
    events::Serpentine,
  },
  tetris::{
    components::{BlockPart, BlockParts, Placed, TetrisBlock},
    events::TetrisMove,
  },
};
use bevy::{
  ecs::query::{ReadOnlyWorldQuery, WorldQuery},
  prelude::{
    Changed, Color, Commands, Component, Entity, EventReader, EventWriter, Or, Query, Res,
    Transform, Vec3, Visibility, With, Without,
  },
};
use rand::random;

pub(super) fn spawn_enemies(mut commands: Commands, game_board: Res<GameBoard>) {
  spawn_single_seeker(Omnivorous, OMNIVOROUS_COLOR, &mut commands, &game_board);
  spawn_single_seeker(Killer, KILLER_COLOR, &mut commands, &game_board);
  spawn_single_seeker(Speedster, SPEEDSTER_COLOR, &mut commands, &game_board);
  spawn_single_seeker(Glutton, GLUTTON_COLOR, &mut commands, &game_board);
}

pub(super) fn respawn(
  mut commands: Commands,
  mut q_dead_enemy: Query<Entity, (Without<Living>, Changed<Visibility>, With<Enemy>)>,
) {
  for enemy in &mut q_dead_enemy {
    commands.entity(enemy).insert(Revive);
  }
}

pub(super) fn seek_food(
  mut serpentine_reader: EventReader<Serpentine>,
  mut q_seeker: Query<&mut Seeker, (With<Enemy>, With<Omnivorous>)>,
  q_target: Query<&Transform, With<Food>>,
) {
  for Serpentine(seeker, head) in serpentine_reader.iter().copied() {
    seek_closest(seeker, &mut q_seeker, &q_target, |food| {
      Some((food.translation.distance(head), food.translation))
    });
  }
}

pub(super) fn seek_snake(
  mut serpentine_reader: EventReader<Serpentine>,
  mut q_seeker: Query<&mut Seeker, (With<Enemy>, With<Killer>)>,
  q_target: Query<(Entity, &Transform, Option<&Food>), Or<(With<Living>, With<Food>)>>,
) {
  for Serpentine(seeker, head) in serpentine_reader.iter().copied() {
    seek_closest(
      seeker,
      &mut q_seeker,
      &q_target,
      |(entity, target, food)| {
        ((food.is_none() && seeker != entity)
          || food.map(|f| *f == Food::Energetic).unwrap_or_default())
        .then_some((target.translation.distance(head), target.translation))
      },
    );
  }
}

pub(super) fn seek_speed(
  mut serpentine_reader: EventReader<Serpentine>,
  mut q_seeker: Query<&mut Seeker, (With<Enemy>, With<Speedster>)>,
  q_target: Query<(&Food, &Transform), With<Food>>,
) {
  for Serpentine(seeker, head) in serpentine_reader.iter().copied() {
    seek_closest(seeker, &mut q_seeker, &q_target, |(food, target)| {
      (*food == Food::Energetic).then_some((target.translation.distance(head), target.translation))
    });
  }
}

pub(super) fn seek_nourishment(
  mut serpentine_reader: EventReader<Serpentine>,
  mut q_seeker: Query<&mut Seeker, (With<Enemy>, With<Glutton>)>,
  q_target: Query<(&Food, &Transform), With<Food>>,
) {
  for Serpentine(seeker, head) in serpentine_reader.iter().copied() {
    seek_closest(seeker, &mut q_seeker, &q_target, |(food, target)| {
      (*food == Food::Beefy).then_some((target.translation.distance(head), target.translation))
    });
  }
}

fn seek_closest<
  C: Component,
  Q: WorldQuery,
  F: ReadOnlyWorldQuery,
  M: FnMut(<<Q as WorldQuery>::ReadOnly as WorldQuery>::Item<'_>) -> Option<(f32, Vec3)>,
>(
  seeker: Entity,
  q_seeker: &mut Query<&mut Seeker, (With<Enemy>, With<C>)>,
  q_target: &Query<Q, F>,
  mut filter_map: M,
) {
  let Ok(mut seeker) = q_seeker.get_mut(seeker) else {return};
  let Some((_, target)) = q_target
    .iter()
    .filter_map(&mut filter_map)
    .min_by(|(a, _), (b, _)| a.partial_cmp(b).unwrap()) else {return};
  seeker.0 = target;
}

fn spawn_single_seeker<C: Component>(
  id_component: C,
  color: Color,
  commands: &mut Commands,
  game_board: &GameBoard,
) {
  let enemy = (
    Enemy,
    id_component,
    Seeker::default(),
    SnakeBundle::new(
      commands,
      SnakeConfig {
        x: (random::<f32>() - 0.5) * game_board.width,
        y: (random::<f32>() - 0.5) * game_board.height,
        color,
        tail_length: INITIAL_ENEMY_LENGTH,
        ..Default::default()
      },
    ),
  );
  commands.spawn(enemy);
}

pub(super) fn tetris_movement(
  mut commands: Commands,
  mut move_writer: EventWriter<TetrisMove>,
  mut q_enemy: Query<
    (Entity, &MoveCooldown, &BlockParts),
    (With<Enemy>, With<TetrisBlock>, Without<TargetLocked>),
  >,
  q_block_parts: Query<&Transform, (With<BlockPart>, Without<Placed>, Without<TetrisBlock>)>,
  q_placed_blocks: Query<&Transform, (With<Placed>, Without<BlockPart>, Without<TetrisBlock>)>,
  game_board: Res<GameBoard>,
) {
  for (enemy, move_cooldown, parts) in &mut q_enemy {
    if !move_cooldown.0.finished() {
      continue;
    }
    let Some(bottom_part) = parts.0.iter()
      .filter_map(|e| q_block_parts.get(*e).ok().map(|t| t.translation))
      .min_by(|a, b| a.y.partial_cmp(&b.y).unwrap()) else {continue};
    let mut cols = iter_cells(0.5 * game_board.width);
    let Some(clear_col) = (loop {
      let Some(x) = cols.next() else {break None};
      if iter_cells_from_to(-0.5 * game_board.height, bottom_part.y)
      .all(|y| {
        let position = Vec3::new(x, y, 0.);
        q_placed_blocks.iter().all(|c| c.translation != position)
      }) {
        break Some(x);
      }
    }) else {continue};
    if clear_col == bottom_part.x {
      commands.entity(enemy).insert(TargetLocked);
      continue;
    } else if clear_col > bottom_part.x {
      move_writer.send(TetrisMove::Right(enemy));
    } else if clear_col < bottom_part.x {
      move_writer.send(TetrisMove::Left(enemy));
    }
  }
}
