use super::components::{Frozen, Invincibility, Swiftness};
use crate::{
  attributes::{
    components::{BaseColor, Brightness, MoveCooldown, Speed},
    utils::{brighten_color, desaturate_color},
  },
  MAX_MOVE_COOLDOWN,
};
use bevy::{
  prelude::{Changed, Commands, Entity, Or, Query, Res},
  sprite::Sprite,
  time::Time,
};

pub(super) fn freeze(
  mut commands: Commands,
  mut q_frozen: Query<(Entity, &mut Frozen)>,
  timer: Res<Time>,
) {
  for (entity, mut frozen) in &mut q_frozen {
    if frozen.finished(timer.delta()) {
      frozen.finish();
      commands.entity(entity).remove::<Frozen>();
    }
  }
}

pub(super) fn invincibility_timer(
  mut commands: Commands,
  mut q_invincibilty: Query<(Entity, &mut Invincibility, &mut Sprite)>,
  timer: Res<Time>,
) {
  for (entity, mut invincibility, mut sprite) in &mut q_invincibilty {
    sprite.color.set_a(0.25);
    if invincibility.finished(timer.delta()) {
      sprite.color.set_a(1.);
      commands.entity(entity).remove::<Invincibility>();
    }
  }
}

pub(super) fn transform_speed(
  mut q_speed: Query<
    (
      &mut MoveCooldown,
      &Speed,
      Option<&Swiftness>,
      Option<&Frozen>,
    ),
    Or<(Changed<Speed>, Changed<Swiftness>, Changed<Frozen>)>,
  >,
) {
  for (mut move_cooldown, speed, swiftness, frozen) in &mut q_speed {
    if let Some(frozen) = frozen {
      if frozen.level() > 0 {
        move_cooldown.set_cooldown_duration(frozen.duration());
        continue;
      }
    }
    let speed = speed.0 * MAX_MOVE_COOLDOWN * 0.6;
    let swiftness = speed * 0.15 * swiftness.map(|s| s.0).unwrap_or_default();
    move_cooldown.set_cooldown_ms((MAX_MOVE_COOLDOWN - (speed + swiftness)) as u64);
  }
}

pub(super) fn transform_color(
  mut q_sprite: Query<
    (
      &mut Sprite,
      &BaseColor,
      &Brightness,
      Option<&Swiftness>,
      Option<&Frozen>,
    ),
    Or<(
      Changed<Brightness>,
      Changed<BaseColor>,
      Changed<Swiftness>,
      Changed<Frozen>,
    )>,
  >,
) {
  for (mut sprite, color, brightness, swiftness, frozen) in &mut q_sprite {
    let frozen = frozen.map(|f| f.level()).unwrap_or_default();
    sprite.color = if frozen > 0 {
      desaturate_color(&color.0, 1. - 1. / (frozen as f32 + 2.))
    } else {
      brighten_color(
        &color.0,
        brightness.0 + 0.5 * swiftness.map(|s| s.0).unwrap_or_default(),
      )
    };
  }
}

pub(super) fn remove_swiftness(
  mut commands: Commands,
  mut q_swiftness: Query<(Entity, &Swiftness), Changed<Swiftness>>,
) {
  for (entity, swiftness) in &mut q_swiftness {
    if swiftness.0 == 0. {
      commands.entity(entity).remove::<Swiftness>();
    }
  }
}
