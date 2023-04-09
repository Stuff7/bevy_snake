use super::{
  components::{Name, Score, ScoreContainer, ScoreValue, Scoreboard},
  events::ScoreUpdate,
  styles,
};
use bevy::{
  prelude::{
    AssetServer, BuildChildren, Changed, Children, Commands, EventWriter, NodeBundle, Query, Res,
    Sprite, TextBundle, With,
  },
  text::Text,
};

pub(super) fn spawn(mut commands: Commands, asset_server: Res<AssetServer>) {
  commands
    .spawn((
      Scoreboard,
      NodeBundle {
        background_color: styles::SCORE_BOARD_BACKGROUND.into(),
        style: styles::SCORE_BOARD,
        ..Default::default()
      },
    ))
    .with_children(|parent| {
      for _ in 0..10 {
        parent
          .spawn((
            ScoreContainer,
            NodeBundle {
              background_color: styles::PLACE_BACKGROUND.into(),
              style: styles::PLACE,
              ..Default::default()
            },
          ))
          .with_children(|parent| {
            parent.spawn((
              ScoreValue,
              TextBundle::from_section("", styles::text(&asset_server)),
            ));
            parent.spawn((
              ScoreValue,
              TextBundle::from_section("", styles::text(&asset_server)),
            ));
          });
      }
    });
}

pub(super) fn update_score(
  mut score_update_writer: EventWriter<ScoreUpdate>,
  mut q_score: Query<(), Changed<Score>>,
) {
  for _ in &mut q_score {
    score_update_writer.send(ScoreUpdate);
  }
}

pub(super) fn update_scoreboard(
  q_score_container: Query<&Children, With<ScoreContainer>>,
  mut q_score_value: Query<&mut Text, With<ScoreValue>>,
  q_score: Query<(&Score, &Name, &Sprite)>,
) {
  let mut scores = q_score
    .iter()
    .map(|(Score(score), Name(name), sprite)| (score, name, sprite.color))
    .collect::<Vec<_>>();
  scores.sort_by(|a, b| b.0.partial_cmp(a.0).unwrap());

  for (i, children) in q_score_container.iter().enumerate() {
    let Some((score, name, color)) = scores.get(i).copied() else {return};
    let mut children = children.iter();

    let Some(score_name) = children.next() else {return};
    let Ok(mut score_name) = q_score_value.get_mut(*score_name) else {return};
    score_name.sections[0].value = name.to_string();
    score_name.sections[0].style.color = color;

    let Some(score_value) = children.next() else {return};
    let Ok(mut score_value) = q_score_value.get_mut(*score_value) else {return};
    score_value.sections[0].value = score.to_string();
    score_value.sections[0].style.color = color;
  }
}
