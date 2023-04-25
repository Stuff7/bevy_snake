use crate::{
  attributes::components::MoveCooldown,
  board::{resources::GameBoard, CELL_SIZE},
  snake::components::SnakeSegment,
};
use bevy::{
  prelude::{Children, Commands, Entity, GlobalTransform, Query, Res, Transform, With, Without},
  time::Time,
};

use super::components::{Placed, TetrisBlock};

pub(super) fn fall(
  mut commands: Commands,
  mut q_blocks: Query<
    (Entity, &mut Transform, &mut MoveCooldown, &Children),
    (With<TetrisBlock>, Without<Placed>),
  >,
  q_segments: Query<(&Transform, &GlobalTransform), (With<SnakeSegment>, Without<TetrisBlock>)>,
  game_board: Res<GameBoard>,
  time: Res<Time>,
) {
  for (entity, mut block, mut move_cooldown, children) in &mut q_blocks {
    for (i, child) in children.iter().enumerate() {
      let Ok((transform, global)) = q_segments.get(*child) else {continue};
      println!("{i} => {transform:?}  ||  {global:?}");
    }
    if !move_cooldown.finished(time.delta()) {
      continue;
    }
    if block.translation.y < CELL_SIZE - game_board.height / 2. {
      commands.entity(entity).insert(Placed);
      continue;
    }
    block.translation.y -= CELL_SIZE;
  }
}
