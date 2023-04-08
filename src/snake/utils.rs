use crate::board::CELL_SIZE;
use bevy::prelude::{Entity, Vec3};

pub fn snake_crashed<H: Iterator<Item = (Entity, Vec3)>, B: Iterator<Item = Vec3>>(
  mut head_iter: H,
  mut body_iter: B,
  snake_entity: Entity,
  snake_head: Vec3,
) -> bool {
  head_iter.any(|(entity, head)| {
    if entity == snake_entity {
      return false;
    }
    head.distance(snake_head) < CELL_SIZE
  }) || body_iter.any(|segment| segment.distance(snake_head) < CELL_SIZE)
}
