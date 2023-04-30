use crate::{
  board::components::Board,
  food::components::Food,
  player::{components::Player, events::RespawnPlayer},
  scoreboard::components::{Name, Score, ScoreEntity},
  snake::{
    components::Snake,
    events::{BodyResize, SnakeResize},
  },
  state::GameState,
  tetris::components::Placed,
};
use bevy::prelude::{
  Children, Commands, Entity, EventWriter, Input, KeyCode, NextState, Query, Res, ResMut, State,
  Transform, With, Without,
};

pub(super) fn god_mode(
  mut respawn_player_writer: EventWriter<RespawnPlayer>,
  mut size_change_writer: EventWriter<SnakeResize>,
  q_player: Query<Entity, With<Player>>,
  keyboard_input: Res<Input<KeyCode>>,
  game_state: Res<State<GameState>>,
  mut next_state: ResMut<NextState<GameState>>,
) {
  use BodyResize::*;
  if keyboard_input.just_pressed(KeyCode::E) {
    let Ok(player) = q_player.get_single() else { return; };
    size_change_writer.send((player, Grow(1)));
  } else if keyboard_input.just_pressed(KeyCode::Q) {
    let Ok(player) = q_player.get_single() else { return; };
    size_change_writer.send((player, Shrink(1)));
  } else if keyboard_input.just_pressed(KeyCode::R) {
    respawn_player_writer.send(RespawnPlayer);
  } else if keyboard_input.just_pressed(KeyCode::P) {
    next_state.set(if game_state.0 == GameState::Paused {
      GameState::Playing
    } else {
      GameState::Paused
    });
  }
}

pub(super) fn move_board(
  mut q_board: Query<&mut Transform, With<Board>>,
  keyboard_input: Res<Input<KeyCode>>,
) {
  let Ok(mut board) = q_board.get_single_mut() else {return};
  if keyboard_input.pressed(KeyCode::Left) {
    board.translation.x -= 1.;
  } else if keyboard_input.pressed(KeyCode::Right) {
    board.translation.x += 1.;
  } else if keyboard_input.pressed(KeyCode::Up) {
    board.translation.y += 1.;
  } else if keyboard_input.pressed(KeyCode::Down) {
    board.translation.y -= 1.;
  }
}

#[allow(clippy::too_many_arguments)]
pub(super) fn print_debug_info(
  mut commands: Commands,
  keyboard_input: Res<Input<KeyCode>>,
  q_entity: Query<Entity>,
  q_scores: Query<&Name, With<Score>>,
  q_snake: Query<&ScoreEntity, With<Snake>>,
  q_placed_blocks: Query<&Transform, (With<Placed>, Without<Food>)>,
  q_food: Query<&Transform, (With<Food>, Without<Placed>)>,
  q_board: Query<&Children, With<Board>>,
) {
  if keyboard_input.just_pressed(KeyCode::O) {
    let Ok(board) = q_board.get_single() else {return};
    let debug = [
      "=== === === DEBUG === === ===",
      &format!("Entity Count: {}", q_entity.iter().count()),
      &format!(
        "Snakes: {:?}",
        q_snake
          .iter()
          .map(|s| q_scores.get(s.0).map(|s| s.0.clone()))
          .collect::<Vec<_>>()
      ),
      &format!(
        "Board children OK!: {:?}",
        board.iter().all(|c| commands.get_entity(*c).is_some())
      ),
      &format!(
        "Placed Blocks: {:?}",
        q_placed_blocks
          .iter()
          .map(|block| block.translation)
          .collect::<Vec<_>>()
      ),
      &format!(
        "Food: {:?}",
        q_food
          .iter()
          .map(|food| food.translation)
          .collect::<Vec<_>>()
      ),
    ]
    .join("\n");
    println!("{debug}");
  }
}
