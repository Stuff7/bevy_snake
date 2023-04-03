use super::{components::Food, events::FoodEaten, FOOD_COLOR};
use crate::{snake::events::BodySizeChange, world::styles::create_cell_bundle};
use bevy::{
  prelude::{Commands, EventReader, EventWriter, Query, Transform, With},
  window::{PrimaryWindow, Window},
};
use rand::random;

pub(super) fn food_spawning(
  mut commands: Commands,
  window: Query<&Window, With<PrimaryWindow>>,
) {
  let window = window.get_single().unwrap();

  commands.spawn((
    Food,
    create_cell_bundle(
      FOOD_COLOR,
      random::<f32>() * window.width(),
      random::<f32>() * window.height(),
    ),
  ));
}

pub(super) fn food_repositioning(
  mut body_size_change_writer: EventWriter<BodySizeChange>,
  mut food_eaten_reader: EventReader<FoodEaten>,
  mut food_query: Query<&mut Transform, With<Food>>,
  window: Query<&Window, With<PrimaryWindow>>,
) {
  let window = window.get_single().unwrap();
  let Ok(mut food) = food_query.get_single_mut() else {
    return;
  };

  for _ in food_eaten_reader.iter() {
    food.translation.x = random::<f32>() * window.width();
    food.translation.y = random::<f32>() * window.height();
    body_size_change_writer.send(BodySizeChange::Grow(1));
  }
}
