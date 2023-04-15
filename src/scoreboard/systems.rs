use super::{
  components::{Place, Score, ScoreValue, Scoreboard},
  events::ScoreUpdate,
  styles,
};
use bevy::{
  prelude::{
    Added, AssetServer, BuildChildren, Changed, Children, Commands, Entity, EventReader,
    EventWriter, NodeBundle, Query, Res, With,
  },
  text::Text,
  time::Time,
  ui::{Style, Val},
};

pub(super) fn spawn_scoreboard(mut commands: Commands) {
  commands.spawn((
    Scoreboard,
    NodeBundle {
      background_color: styles::SCOREBOARD_BACKGROUND.into(),
      style: styles::SCOREBOARD,
      ..Default::default()
    },
  ));
}

pub(super) fn add_score(
  mut commands: Commands,
  q_scoreboard: Query<Entity, With<Scoreboard>>,
  mut q_scores: Query<(Entity, &Children), Added<Score>>,
  mut q_texts: Query<&mut Text>,
  asset_server: Res<AssetServer>,
) {
  let Ok(scoreboard) = q_scoreboard.get_single() else {return};
  for (score, children) in &mut q_scores {
    commands.entity(scoreboard).add_child(score);
    for child in children {
      let Ok(mut text) = q_texts.get_mut(*child) else {return};
      text.sections[0].style.font = asset_server.load("fonts/UbuntuMono-Regular.ttf");
    }
  }
}

pub(super) fn update_score(
  mut q_score_values: Query<&mut Text, With<ScoreValue>>,
  mut q_scores: Query<(&Score, &Children), Changed<Score>>,
  mut score_update_writer: EventWriter<ScoreUpdate>,
) {
  for (score, children) in &mut q_scores {
    let Some(score_value) = children.get(1) else {return};
    let Ok(mut score_value) = q_score_values.get_mut(*score_value) else {return};
    score_value.sections[0].value = score.0.to_string();
    score_update_writer.send(ScoreUpdate);
  }
}

pub(super) fn sort_scores(
  mut q_scores: Query<(&Score, &mut Place)>,
  mut score_update_reader: EventReader<ScoreUpdate>,
) {
  for _ in &mut score_update_reader {
    let mut scores = q_scores
      .iter_mut()
      .map(|(Score(score), place)| (score, place))
      .collect::<Vec<_>>();
    scores.sort_by(|a, b| b.0.partial_cmp(a.0).unwrap());

    for (i, (_, ref mut place)) in scores.iter_mut().enumerate() {
      place.0 = (i + 1) as f32 * styles::SCORE_HEIGHT;
    }
  }
}

pub(super) fn position_scores(mut q_scores: Query<(&Place, &mut Style)>, time: Res<Time>) {
  for (place, mut style) in &mut q_scores {
    let Val::Px(ref mut top) = style.position.top else {return};
    if *top == place.0 {
      continue;
    }
    let units = f32::min(250. * time.delta_seconds(), (place.0 - *top).abs());
    *top += if *top < place.0 { units } else { -units };
  }
}
