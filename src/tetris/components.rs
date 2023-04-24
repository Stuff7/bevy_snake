use bevy::{
  ecs::system::EntityCommands,
  prelude::{BuildChildren, Bundle, Component, Entity, SpatialBundle, Transform},
};

#[derive(Debug, Component)]
pub struct TetrisBlock;

#[derive(Debug, Bundle)]
pub struct TetrisBlockBundle {
  block: TetrisBlock,
  #[bundle]
  spatial_bundle: SpatialBundle,
}

impl TetrisBlockBundle {
  pub fn insert_to(mut entity: EntityCommands, transform: Transform, children: &[Entity]) {
    entity
      .insert(Self {
        block: TetrisBlock,
        spatial_bundle: SpatialBundle {
          transform,
          ..Default::default()
        },
      })
      .push_children(children);
  }
}
