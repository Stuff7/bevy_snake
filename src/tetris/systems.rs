use crate::{
  attributes::components::{BaseColor, Brightness, MoveCooldown, Speed},
  board::{resources::GameBoard, CELL_SIZE},
  effects::components::Invincibility,
  scoreboard::components::{Score, ScoreEntity},
  snake::components::{SnakeBundle, SnakeConfig, Snakified},
};
use bevy::{
  prelude::{
    Commands, Entity, EventReader, EventWriter, Query, Res, Transform, Vec3, With, Without,
  },
  time::Time,
};
use rand::random;

use super::{
  components::{BlockPart, BlockParts, TetrisBlock, TetrisBlockBundle},
  events::TetrisMove,
};

pub(super) fn fall(
  mut move_writer: EventWriter<TetrisMove>,
  mut q_blocks: Query<(Entity, &mut MoveCooldown), (With<TetrisBlock>, Without<Snakified>)>,
  time: Res<Time>,
) {
  for (entity, mut move_cooldown) in &mut q_blocks {
    if !move_cooldown.finished(time.delta()) {
      continue;
    }
    move_writer.send(TetrisMove::Down(entity));
  }
}

pub(super) fn place(
  mut commands: Commands,
  q_blocks: Query<(Entity, &BlockParts), (With<TetrisBlock>, Without<Snakified>)>,
  q_block_parts: Query<&Transform, (With<BlockPart>, Without<TetrisBlock>)>,
  game_board: Res<GameBoard>,
) {
  for (entity, parts) in &q_blocks {
    for part in &parts.0 {
      let Ok(part) = q_block_parts.get(*part) else {continue};
      if part.translation.y < CELL_SIZE - game_board.height / 2. {
        commands.entity(entity).insert(Snakified);
        return;
      }
    }
  }
}

pub(super) fn snakify(
  mut commands: Commands,
  mut q_blocks: Query<
    (Entity, &ScoreEntity, &Speed, &BaseColor, &Brightness),
    (With<TetrisBlock>, With<Snakified>),
  >,
  mut q_scores: Query<&mut Score>,
  game_board: Res<GameBoard>,
) {
  for (block, score_entity, speed, color, brightness) in &mut q_blocks {
    let Ok(ref mut score) = q_scores.get_mut(score_entity.0) else {return};
    score.0 += 5;
    let snake_bundle = SnakeBundle::new(
      &mut commands,
      SnakeConfig {
        score: Some(score_entity.0),
        tail_length: score.0,
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
      .insert((snake_bundle, Invincibility::new()));
  }
}

pub(super) fn move_parts(
  mut move_reader: EventReader<TetrisMove>,
  mut q_blocks: Query<&BlockParts, With<TetrisBlock>>,
  mut q_block_parts: Query<&mut Transform, (With<BlockPart>, Without<TetrisBlock>)>,
) {
  for movement in &mut move_reader {
    let (entity, translation) = match *movement {
      TetrisMove::Down(block) => (block, Vec3::new(0., -CELL_SIZE, 0.)),
      TetrisMove::Left(block) => (block, Vec3::new(-CELL_SIZE, 0., 0.)),
      TetrisMove::Right(block) => (block, Vec3::new(CELL_SIZE, 0., 0.)),
    };
    let Ok(parts) = q_blocks.get_mut(entity) else {continue};
    for part in &parts.0 {
      let Ok(mut part) = q_block_parts.get_mut(*part) else {continue};
      part.translation += translation;
    }
  }
}
