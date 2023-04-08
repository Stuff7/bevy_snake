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
  use bevy::prelude::{Camera2dBundle, Commands};

  pub(super) fn spawn(mut commands: Commands) {
    commands.spawn((MainCamera, Camera2dBundle::default()));
  }
}

pub mod components {
  use bevy::prelude::Component;

  #[derive(Debug, Component)]
  pub struct MainCamera;
}
