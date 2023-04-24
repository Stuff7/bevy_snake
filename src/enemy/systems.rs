use super::{
  components::{Eater, Enemy, Glutton, Killer, Speedster},
  EATER_COLOR, GLUTTON_COLOR, INITIAL_ENEMY_LENGTH, KILLER_COLOR, SPEEDSTER_COLOR,
};
use crate::{
  board::{components::Board, resources::GameBoard},
  food::components::Food,
  snake::{
    components::{Living, Revive, Seeker, SnakeBundle, SnakeConfig},
    events::Serpentine,
  },
};
use bevy::{
  ecs::query::{ReadOnlyWorldQuery, WorldQuery},
  prelude::{
    BuildChildren, Changed, Color, Commands, Component, Entity, EventReader, Or, Query, Res,
    Transform, Vec3, Visibility, With, Without,
  },
};
use rand::random;

pub(super) fn spawn_enemies(
  mut commands: Commands,
  q_board: Query<Entity, With<Board>>,
  game_board: Res<GameBoard>,
) {
  spawn_single_seeker(Eater, EATER_COLOR, &mut commands, &q_board, &game_board);
  spawn_single_seeker(Killer, KILLER_COLOR, &mut commands, &q_board, &game_board);
  spawn_single_seeker(
    Speedster,
    SPEEDSTER_COLOR,
    &mut commands,
    &q_board,
    &game_board,
  );
  spawn_single_seeker(Glutton, GLUTTON_COLOR, &mut commands, &q_board, &game_board);
}

pub(super) fn respawn(
  mut commands: Commands,
  mut q_dead_enemy: Query<Entity, (Without<Living>, Changed<Visibility>, With<Enemy>)>,
) {
  for enemy in &mut q_dead_enemy {
    commands.entity(enemy).insert(Revive);
  }
}

pub(super) fn seek_food(
  mut serpentine_reader: EventReader<Serpentine>,
  mut q_seeker: Query<&mut Seeker, (With<Enemy>, With<Eater>)>,
  q_target: Query<&Transform, With<Food>>,
) {
  for Serpentine(seeker, head) in serpentine_reader.iter().copied() {
    seek_closest(seeker, &mut q_seeker, &q_target, |food| {
      Some((food.translation.distance(head), food.translation))
    });
  }
}

pub(super) fn seek_snake(
  mut serpentine_reader: EventReader<Serpentine>,
  mut q_seeker: Query<&mut Seeker, (With<Enemy>, With<Killer>)>,
  q_target: Query<(Entity, &Transform, Option<&Food>), Or<(With<Living>, With<Food>)>>,
) {
  for Serpentine(seeker, head) in serpentine_reader.iter().copied() {
    seek_closest(
      seeker,
      &mut q_seeker,
      &q_target,
      |(entity, target, food)| {
        ((food.is_none() && seeker != entity)
          || food.map(|f| *f == Food::Energetic).unwrap_or_default())
        .then_some((target.translation.distance(head), target.translation))
      },
    );
  }
}

pub(super) fn seek_speed(
  mut serpentine_reader: EventReader<Serpentine>,
  mut q_seeker: Query<&mut Seeker, (With<Enemy>, With<Speedster>)>,
  q_target: Query<(&Food, &Transform), With<Food>>,
) {
  for Serpentine(seeker, head) in serpentine_reader.iter().copied() {
    seek_closest(seeker, &mut q_seeker, &q_target, |(food, target)| {
      (*food == Food::Energetic).then_some((target.translation.distance(head), target.translation))
    });
  }
}

pub(super) fn seek_nourishment(
  mut serpentine_reader: EventReader<Serpentine>,
  mut q_seeker: Query<&mut Seeker, (With<Enemy>, With<Glutton>)>,
  q_target: Query<(&Food, &Transform), With<Food>>,
) {
  for Serpentine(seeker, head) in serpentine_reader.iter().copied() {
    seek_closest(seeker, &mut q_seeker, &q_target, |(food, target)| {
      (*food == Food::Beefy).then_some((target.translation.distance(head), target.translation))
    });
  }
}

fn seek_closest<
  C: Component,
  Q: WorldQuery,
  F: ReadOnlyWorldQuery,
  M: FnMut(<<Q as WorldQuery>::ReadOnly as WorldQuery>::Item<'_>) -> Option<(f32, Vec3)>,
>(
  seeker: Entity,
  q_seeker: &mut Query<&mut Seeker, (With<Enemy>, With<C>)>,
  q_target: &Query<Q, F>,
  mut filter_map: M,
) {
  let Ok(mut seeker) = q_seeker.get_mut(seeker) else {return};
  let Some((_, target)) = q_target
    .iter()
    .filter_map(&mut filter_map)
    .min_by(|(a, _), (b, _)| a.partial_cmp(b).unwrap()) else {return};
  seeker.0 = target;
}

fn spawn_single_seeker<C: Component>(
  id_component: C,
  color: Color,
  commands: &mut Commands,
  q_board: &Query<Entity, With<Board>>,
  game_board: &GameBoard,
) {
  let Ok(board) = q_board.get_single() else {return};
  let enemy = (
    Enemy,
    id_component,
    Seeker::default(),
    SnakeBundle::new(
      commands,
      board,
      SnakeConfig {
        x: (random::<f32>() - 0.5) * game_board.width,
        y: (random::<f32>() - 0.5) * game_board.height,
        color,
        tail_length: INITIAL_ENEMY_LENGTH,
        ..Default::default()
      },
    ),
  );
  let enemy = commands.spawn(enemy).id();
  commands.entity(board).add_child(enemy);
}
