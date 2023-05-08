mod systems;

use bevy::prelude::{in_state, App, IntoSystemConfig, Plugin};

use crate::state::GameState;

pub struct EffectPlugin;

impl Plugin for EffectPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_system(systems::freeze.run_if(in_state(GameState::Playing)))
      .add_system(systems::remove_swiftness)
      .add_system(systems::transform_speed)
      .add_system(systems::transform_color)
      .add_system(systems::invincibility_timer);
  }
}

pub mod components {
  use bevy::{
    prelude::Component,
    time::{Timer, TimerMode},
  };
  use std::time::Duration;

  const FROZEN_SECONDS: u64 = 1;
  const INVINCIBILITY_SECONDS: u64 = 3;

  #[derive(Debug, Component, Default)]
  pub struct Swiftness(pub f32);

  #[derive(Debug, Component, Default)]
  pub struct Frozen {
    level: u64,
    cooldown: Timer,
  }

  impl Frozen {
    pub fn new() -> Self {
      Self {
        level: 1,
        cooldown: Timer::new(Duration::from_millis(FROZEN_SECONDS), TimerMode::Once),
      }
    }

    pub fn level(&self) -> u64 {
      self.level
    }

    pub fn finish(&mut self) {
      self.level = 0;
    }

    pub fn duration(&self) -> Duration {
      self.cooldown.duration()
    }

    pub fn increase_level(&mut self) {
      if self.level < 3 {
        self.level += 1;
        self
          .cooldown
          .set_duration(Duration::from_secs(FROZEN_SECONDS + self.level));
      }
      self.cooldown.reset();
    }

    pub fn finished(&mut self, delta: Duration) -> bool {
      self.cooldown.tick(delta);
      self.cooldown.finished()
    }
  }

  #[derive(Debug, Component)]
  pub struct Invincibility(Timer);

  impl Invincibility {
    pub fn new() -> Self {
      Self(Timer::new(
        Duration::from_secs(INVINCIBILITY_SECONDS),
        TimerMode::Once,
      ))
    }

    pub fn finished(&mut self, delta: Duration) -> bool {
      self.0.tick(delta);
      self.0.finished()
    }
  }
}
