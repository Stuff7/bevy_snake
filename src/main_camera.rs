use bevy::prelude::{App, ClearColor, Color, Plugin};

pub struct MainCameraPlugin;

pub const BACKGROUND_COLOR: Color = Color::rgb(8. / 255., 8. / 255., 8. / 255.);

impl Plugin for MainCameraPlugin {
  fn build(&self, app: &mut App) {
    app
      .insert_resource(ClearColor(BACKGROUND_COLOR))
      .add_startup_system(systems::spawn);
  }
}

mod systems {
  use super::components::MainCamera;
  use bevy::{
    core_pipeline::{
      bloom::{BloomCompositeMode, BloomPrefilterSettings, BloomSettings},
      tonemapping::Tonemapping,
    },
    prelude::{Camera, Camera2dBundle, Commands},
  };

  pub(super) fn spawn(mut commands: Commands) {
    commands.spawn((
      MainCamera,
      Camera2dBundle {
        camera: Camera {
          hdr: true,
          ..Default::default()
        },
        tonemapping: Tonemapping::TonyMcMapface,
        ..Default::default()
      },
      BloomSettings {
        composite_mode: BloomCompositeMode::Additive,
        high_pass_frequency: 1.,
        intensity: 0.05,
        low_frequency_boost: 1.,
        low_frequency_boost_curvature: 1.,
        prefilter_settings: BloomPrefilterSettings {
          threshold: 1.,
          ..Default::default()
        },
      },
    ));
  }
}

pub mod components {
  use bevy::prelude::Component;

  #[derive(Debug, Component)]
  pub struct MainCamera;
}
