use crate::{
  attributes::components::{BaseColor, Brightness, MoveCooldown, Speed},
  board::{
    components::{CellCollider, DyingCell, RandomCellPosition},
    resources::GameBoard,
    utils::iter_cells,
    CELL_SIZE,
  },
  effects::components::Invincibility,
  enemy::components::EnemyTetrisCleanupBundle,
  food::components::Food,
  scoreboard::components::{Score, ScoreEntity},
  snake::components::{SnakeBundle, SnakeConfig, Snakified},
};
use bevy::{
  prelude::{
    Commands, Entity, EventReader, EventWriter, Query, Res, ResMut, Transform, Vec3, With, Without,
  },
  time::Time,
};
use rand::random;

use super::{
  components::{BlockPart, BlockParts, FreeFall, Placed, TetrisBlock, TetrisBlockBundle},
  events::{TetrisMove, TetrisPlace},
  resources::FallTimer,
};

#[allow(clippy::too_many_arguments)]
pub(super) fn fall(
  mut commands: Commands,
  mut move_writer: EventWriter<TetrisMove>,
  mut q_blocks: Query<
    (Entity, &mut MoveCooldown, &BlockParts),
    (With<TetrisBlock>, Without<Snakified>),
  >,
  q_block_parts: Query<&Transform, (With<BlockPart>, Without<TetrisBlock>)>,
  mut q_placed_blocks: Query<&Transform, (With<Placed>, Without<BlockPart>)>,
  game_board: Res<GameBoard>,
  time: Res<Time>,
  fall_cooldown: Res<FallTimer>,
) {
  for (entity, mut move_cooldown, parts) in &mut q_blocks {
    move_cooldown.0.tick(time.delta());
    if !fall_cooldown.0.finished() {
      continue;
    }
    for part in &parts.0 {
      let Ok(part) = q_block_parts.get(*part) else {continue};
      if part.translation.y < CELL_SIZE - game_board.height / 2. {
        commands.entity(entity).insert(Snakified);
        return;
      } else {
        let next_translation = part.translation + Vec3::new(0., -CELL_SIZE, 0.);
        for surface in &mut q_placed_blocks {
          if surface.translation == next_translation {
            commands.entity(entity).insert(Snakified);
            return;
          }
        }
      }
    }
    move_writer.send(TetrisMove::Down(entity));
  }
}

pub(super) fn gravity(
  mut commands: Commands,
  q_placed_blocks: Query<(Entity, &Transform), With<Placed>>,
  game_board: Res<GameBoard>,
  time: Res<Time>,
  mut fall_cooldown: ResMut<FallTimer>,
) {
  if !fall_cooldown.finished(time.delta()) {
    return;
  }
  for (entity, block) in &q_placed_blocks {
    let half_height = game_board.height * 0.5;
    let y = block.translation.y - CELL_SIZE;
    if y < -half_height || q_placed_blocks.iter().any(|(_, t)| t.translation.y == y) {
      continue;
    }
    commands.entity(entity).insert(FreeFall);
  }
}

pub(super) fn free_fall(
  mut commands: Commands,
  mut q_falling_blocks: Query<(Entity, &mut Transform), (With<Placed>, With<FreeFall>)>,
) {
  for (entity, mut block) in &mut q_falling_blocks {
    block.translation.y -= CELL_SIZE;
    commands.entity(entity).remove::<FreeFall>();
  }
}

pub(super) fn move_parts(
  mut commands: Commands,
  mut move_reader: EventReader<TetrisMove>,
  mut q_blocks: Query<&BlockParts, (With<TetrisBlock>, Without<Food>)>,
  mut q_block_parts: Query<
    &mut Transform,
    (
      With<BlockPart>,
      Without<Placed>,
      Without<TetrisBlock>,
      Without<Food>,
    ),
  >,
  q_cells: Query<&Transform, (With<CellCollider>, Without<BlockPart>, Without<Food>)>,
  q_food: Query<(Entity, &Transform), (With<Food>, Without<TetrisBlock>, Without<BlockPart>)>,
  game_board: Res<GameBoard>,
) {
  for movement in &mut move_reader {
    let (entity, translation) = match *movement {
      TetrisMove::Down(block) => (block, Vec3::new(0., -CELL_SIZE, 0.)),
      TetrisMove::Left(block) => (block, Vec3::new(-CELL_SIZE, 0., 0.)),
      TetrisMove::Right(block) => (block, Vec3::new(CELL_SIZE, 0., 0.)),
    };
    let Ok(parts) = q_blocks.get_mut(entity) else {continue};
    let half_width = game_board.width * 0.5;
    let half_height = game_board.height * 0.5;
    let parts_translations = parts
      .0
      .iter()
      .filter_map(|e| q_block_parts.get(*e).ok().map(|t| t.translation))
      .collect::<Vec<_>>();
    if parts_translations.iter().any(|p| {
      let t = *p + translation;
      t.x > half_width || t.x < -half_width || t.y < -half_height || {
        q_cells
          .iter()
          .chain(q_block_parts.iter())
          .filter(|c| !parts_translations.contains(&c.translation))
          .any(|c| c.translation == t)
      }
    }) {
      continue;
    }
    for part in &parts.0 {
      let Ok(mut part) = q_block_parts.get_mut(*part) else {continue};
      part.translation += translation;
      if let Some((food, _)) = q_food
        .iter()
        .find(|(_, food)| part.translation == food.translation)
      {
        commands.entity(food).insert(RandomCellPosition);
      }
    }
  }
}

pub(super) fn snakify(
  mut commands: Commands,
  mut place_writer: EventWriter<TetrisPlace>,
  mut q_blocks: Query<
    (
      Entity,
      &BlockParts,
      &ScoreEntity,
      &Speed,
      &BaseColor,
      &Brightness,
    ),
    (With<TetrisBlock>, With<Snakified>),
  >,
  q_parts: Query<&Transform, With<BlockPart>>,
  mut q_scores: Query<&mut Score>,
  game_board: Res<GameBoard>,
) {
  for (block, parts, score_entity, speed, color, brightness) in &mut q_blocks {
    let rows = parts
      .0
      .iter()
      .filter_map(|part| {
        commands.entity(*part).remove::<BlockPart>().insert(Placed);
        q_parts.get(*part).ok().map(|t| (*part, *t))
      })
      .collect::<Vec<_>>();
    place_writer.send(TetrisPlace(score_entity.0, rows));
    let Ok(ref mut score) = q_scores.get_mut(score_entity.0) else {return};
    score.0 += 5;
    let snake_bundle = SnakeBundle::new(
      &mut commands,
      SnakeConfig {
        score: Some(score_entity.0),
        speed: speed.0,
        color: color.0,
        brightness: brightness.0,
        x: (random::<f32>() - 0.5) * game_board.width,
        y: (random::<f32>() - 0.5) * game_board.height,
        ..Default::default()
      },
    );
    commands
      .entity(block)
      .remove::<TetrisBlockBundle>()
      .remove::<Snakified>()
      .remove::<EnemyTetrisCleanupBundle>()
      .insert((snake_bundle, Invincibility::new()));
  }
}

pub(super) fn clear_line(
  mut commands: Commands,
  mut place_reader: EventReader<TetrisPlace>,
  q_placed_blocks: Query<(Entity, &Transform), With<Placed>>,
  mut q_scores: Query<&mut Score>,
  game_board: Res<GameBoard>,
) {
  for TetrisPlace(who, rows) in &mut place_reader {
    if rows.is_empty() {
      continue;
    }
    for row in rows {
      let Ok(cells) = iter_cells(0.5 * game_board.width).map(|x| {
        q_placed_blocks
          .iter()
          .chain(std::iter::once((row.0, &row.1)))
          .find(|(_, t)| t.translation.x == x && t.translation.y == row.1.translation.y).ok_or(())
      }).collect::<Result<Vec<_>, _>>() else {continue};
      let Ok(mut score) = q_scores.get_mut(*who) else {continue};
      score.0 += cells.len() as i32;
      for (cell, _) in cells {
        commands.entity(cell).insert(DyingCell);
      }
    }
  }
}
