use crate::board::CELL_SIZE;
use bevy::prelude::{Entity, Vec3};

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
