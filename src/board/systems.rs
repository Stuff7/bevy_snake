use super::{
  components::{Board, BoardSprite, Cell, DyingCell, RandomCellPosition},
  resources::GameBoard,
  utils::iter_cells,
  BOARD_COLOR, CELL_SIZE, HALF_CELL_SIZE,
};
use bevy::{
  prelude::{
    Added, BuildChildren, Children, Commands, DetectChanges, Entity, EventReader, Parent, Query,
    Res, ResMut, SpatialBundle, Sprite, SpriteBundle, Transform, Vec2, Vec3, Visibility, With,
    Without,
  },
  window::{PrimaryWindow, Window, WindowResized},
};
use rand::random;

pub(super) fn spawn(
  mut commands: Commands,
  window: Query<&Window, With<PrimaryWindow>>,
  mut game_board: ResMut<GameBoard>,
) {
  let window = window.get_single().unwrap();
  game_board.resize(window.width(), window.height());

  let board = commands
    .spawn((
      BoardSprite,
      SpriteBundle {
        sprite: Sprite {
          color: BOARD_COLOR,
          custom_size: Some(Vec2::ZERO),
          ..Default::default()
        },
        ..Default::default()
      },
    ))
    .id();

  commands
    .spawn((Board, SpatialBundle::default()))
    .add_child(board);
}

pub(super) fn add_cell(
  mut commands: Commands,
  q_board: Query<Entity, With<Board>>,
  mut q_cells: Query<(Entity, &mut Visibility), Added<Cell>>,
) {
  for (cell, mut visibility) in &mut q_cells {
    let Ok(board) = q_board.get_single() else {return};
    commands.entity(board).add_child(cell);
    *visibility = Visibility::Visible;
  }
}

pub(super) fn remove_cell(
  mut commands: Commands,
  q_board: Query<Entity, With<Board>>,
  mut q_cells: Query<Entity, Added<DyingCell>>,
) {
  for cell in &mut q_cells {
    let Ok(board) = q_board.get_single() else {return};
    commands.entity(board).remove_children(&[cell]);
    commands.entity(cell).despawn();
  }
}

pub(super) fn position_cell_randomly(
  mut commands: Commands,
  mut q_repositioned_cells: Query<(Entity, &mut Transform), Added<RandomCellPosition>>,
  q_cells: Query<&Transform, (With<Cell>, Without<DyingCell>, Without<RandomCellPosition>)>,
  game_board: Res<GameBoard>,
) {
  for (entity, mut cell) in &mut q_repositioned_cells {
    let mut rows = iter_cells(0.5 * game_board.height).collect::<Vec<_>>();
    let mut cells = q_cells.iter();
    commands.entity(entity).remove::<RandomCellPosition>();
    let Some(new_position) = (loop {
      if rows.is_empty() {
        break None;
      }
      let y = rows.remove(random::<usize>() % rows.len());
      let mut available_cells = iter_cells(0.5 * game_board.width)
        .filter_map(|x| {
          let position = Vec3::new(x, y, 0.);
          (!cells.any(|c| {
            c.translation == position ||
            c.translation + Vec3::new(-CELL_SIZE, 0., 0.) == position ||
            c.translation + Vec3::new(CELL_SIZE, 0., 0.) == position
          })).then_some(position)
        })
        .collect::<Vec<_>>();
      if !available_cells.is_empty() {
        break Some(available_cells.remove(random::<usize>() % available_cells.len()));
      }
    }) else {println!("COULD NOT FIND EMPTY SPOTS"); return};
    cell.translation = new_position;
  }
}

pub(super) fn resize_game_board(
  mut resize_reader: EventReader<WindowResized>,
  mut q_board_sprite: Query<&mut Sprite, With<BoardSprite>>,
  mut q_board_position: Query<&mut Transform, With<Board>>,
  mut game_board: ResMut<GameBoard>,
) {
  for resize in &mut resize_reader {
    let Ok(mut board_transform) = q_board_position.get_single_mut() else {return};
    let Ok(mut board_sprite) = q_board_sprite.get_single_mut() else {return};
    let Some(ref mut board_sprite) = board_sprite.custom_size else {return};
    board_transform.translation.x = resize.width * 0.1;
    game_board.resize(resize.width, resize.height);
    board_sprite.x = game_board.width;
    board_sprite.y = game_board.height;
  }
}

pub(super) fn constraint_children(
  q_board: Query<&Children, With<Board>>,
  mut q_children: Query<&mut Transform, With<Parent>>,
  game_board: Res<GameBoard>,
) {
  if game_board.is_changed() {
    let Ok(children) = q_board.get_single() else {return};
    for child in children.iter() {
      let Ok(mut child) = q_children.get_mut(*child) else {return};
      if child.translation.x > game_board.width / 2. {
        child.translation.x = game_board.width / 2. - HALF_CELL_SIZE;
      } else if child.translation.x < game_board.width / -2. {
        child.translation.x = HALF_CELL_SIZE - game_board.width / 2.;
      }
      if child.translation.y > game_board.height / 2. {
        child.translation.y = game_board.height / 2. - HALF_CELL_SIZE;
      } else if child.translation.y < game_board.height / -2. {
        child.translation.y = HALF_CELL_SIZE - game_board.height / 2.;
      }
    }
  }
}
