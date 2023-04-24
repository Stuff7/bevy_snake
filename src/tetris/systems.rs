use crate::board::CELL_SIZE;
use bevy::prelude::{Query, Transform, With};

use super::components::TetrisBlock;

pub(super) fn fall(mut q_blocks: Query<&mut Transform, With<TetrisBlock>>) {
  for mut block in &mut q_blocks {
    block.translation.y -= CELL_SIZE;
  }
}
