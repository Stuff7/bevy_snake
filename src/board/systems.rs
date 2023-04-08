use bevy::{
  core_pipeline::clear_color::ClearColorConfig,
  prelude::{
    Assets, Camera, Camera2d, Camera2dBundle, Commands, Image, Query, ResMut, Transform,
    UiCameraConfig, Vec3, With,
  },
  render::{
    camera::RenderTarget,
    render_resource::{
      Extent3d, TextureDescriptor, TextureDimension, TextureFormat, TextureUsages,
    },
  },
  sprite::SpriteBundle,
  window::{PrimaryWindow, Window},
};

use super::{
  components::{Board, MainCamera},
  BOARD_COLOR, CELL_SIZE,
};

pub(super) fn spawn_camera(
  mut commands: Commands,
  window: Query<&Window, With<PrimaryWindow>>,
  mut images: ResMut<Assets<Image>>,
) {
  let window = window.get_single().unwrap();
  let width = window.width();
  let height = window.height();
  let main_camera_position = Vec3::new(-9999., -9999., 0.);

  commands.spawn((
    MainCamera,
    Camera2dBundle {
      transform: Transform::from_translation(main_camera_position),
      ..Default::default()
    },
  ));

  let size = Extent3d {
    width: ((width / CELL_SIZE).floor() * CELL_SIZE) as u32,
    height: ((height / CELL_SIZE).floor() * CELL_SIZE) as u32,
    ..Default::default()
  };

  // This is the texture that will be rendered to.
  let mut image = Image {
    texture_descriptor: TextureDescriptor {
      label: None,
      size,
      dimension: TextureDimension::D2,
      format: TextureFormat::Bgra8UnormSrgb,
      mip_level_count: 1,
      sample_count: 1,
      usage: TextureUsages::TEXTURE_BINDING
        | TextureUsages::COPY_DST
        | TextureUsages::RENDER_ATTACHMENT,
      view_formats: &[],
    },
    ..Default::default()
  };

  // fill image.data with zeroes
  image.resize(size);

  let image_handle = images.add(image);

  commands.spawn((
    Board,
    SpriteBundle {
      texture: image_handle.clone(),
      transform: Transform::from_translation(main_camera_position),
      ..Default::default()
    },
  ));

  commands.spawn((
    UiCameraConfig { show_ui: false },
    Camera2dBundle {
      camera: Camera {
        order: 1,
        target: RenderTarget::Image(image_handle),
        ..Default::default()
      },
      camera_2d: Camera2d {
        clear_color: ClearColorConfig::Custom(BOARD_COLOR),
      },
      ..Default::default()
    },
  ));
}
