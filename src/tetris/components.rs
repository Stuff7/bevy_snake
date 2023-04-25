use bevy::{
  ecs::system::EntityCommands,
  prelude::{BuildChildren, Bundle, Component, Entity, Transform},
  sprite::{Sprite, SpriteBundle},
};

use crate::attributes::components::SpeedBundle;

#[derive(Debug, Component)]
pub struct TetrisBlock;

#[derive(Debug, Component)]
pub struct Tetrified;

#[derive(Debug, Component)]
pub struct Placed;

#[derive(Bundle)]
pub struct TetrisBlockBundle {
  block: TetrisBlock,
  #[bundle]
  speed_bundle: SpeedBundle,
  #[bundle]
  sprite_bundle: SpriteBundle,
}

impl TetrisBlockBundle {
  pub fn insert_to(
    entity: &mut EntityCommands,
    (head_transform, head_sprite): (Transform, Sprite),
    speed: f32,
    children: &[Entity],
  ) {
    entity
      .insert(Self {
        block: TetrisBlock,
        speed_bundle: SpeedBundle::new(speed),
        sprite_bundle: SpriteBundle {
          transform: head_transform,
          sprite: head_sprite,
          ..Default::default()
        },
      })
      .push_children(children);
  }
}
