use crate::{
  board::{resources::GameBoard, utils::get_board_position, CELL_SIZE},
  collections::ExternalOps,
};
use bevy::prelude::{Commands, Entity, Transform, Vec3, Visibility};
use rand::random;

use super::{
  components::{Direction, Living, Nourished, Speed},
  SERPENTINE_DURATION,
};

pub const SNAKE_NAMES: [&str; 50] = [
  "Slytherin",
  "Serpentine",
  "Noodle",
  "Fang",
  "Scales",
  "Hiss",
  "Viper",
  "Cobra",
  "Python",
  "Rattles",
  "Basilisk",
  "Kaa",
  "Slinky",
  "Slippy",
  "Scaly",
  "Anaconda",
  "Medusa",
  "Sidewinder",
  "Boa",
  "Adder",
  "Monty",
  "Garter",
  "Coil",
  "Stripes",
  "Fangs",
  "Venom",
  "Constrictor",
  "Mamba",
  "Diamondback",
  "Zara",
  "Quetzalcoatl",
  "Slither",
  "Twister",
  "Sly",
  "Slink",
  "Glider",
  "Glissando",
  "Marauder",
  "Leviathan",
  "Nachash",
  "Wraps",
  "Wriggles",
  "Twists",
  "Tailspin",
  "Slinky",
  "Screech",
  "Sizzle",
  "Shaker",
  "Vicious",
  "Jaws",
];

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

pub fn sort_direction_by_nearest(
  position: Vec3,
  target: Vec3,
  game_board: &GameBoard,
) -> [Direction; 4] {
  use Direction::*;
  let direction_h = if position.x > target.x { Left } else { Right };
  let (x, y) = (position.x, position.y).add(direction_h.xy(CELL_SIZE, CELL_SIZE));
  let distance_h = target.distance(Vec3::new(x, y, 0.));

  let direction_v = if position.y > target.y { Bottom } else { Top };
  let (x, y) = (position.x, position.y).add(direction_v.xy(CELL_SIZE, CELL_SIZE));
  let distance_v = target.distance(Vec3::new(x, y, 0.));

  if distance_h < distance_v {
    if distance_h > game_board.width / 2. {
      [
        direction_h.opposite(),
        direction_h,
        direction_v,
        direction_v.opposite(),
      ]
    } else {
      [
        direction_h,
        direction_v,
        direction_v.opposite(),
        direction_h.opposite(),
      ]
    }
  } else if distance_v > game_board.height / 2. {
    [
      direction_v.opposite(),
      direction_v,
      direction_h,
      direction_h.opposite(),
    ]
  } else {
    [
      direction_v,
      direction_h,
      direction_h.opposite(),
      direction_v.opposite(),
    ]
  }
}

pub fn revive_snake(
  commands: &mut Commands,
  (snake, visibility, transform, speed): (Entity, &mut Visibility, &mut Transform, &mut Speed),
  game_board: &GameBoard,
) {
  transform.translation = get_board_position(
    (random::<f32>() - 0.5) * game_board.width,
    (random::<f32>() - 0.5) * game_board.height,
  );
  *visibility = Visibility::Visible;
  speed.set_duration(SERPENTINE_DURATION);
  commands.entity(snake).insert(Living).insert(Nourished(4));
}
