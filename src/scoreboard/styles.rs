use bevy::{
  prelude::{
    AlignItems, Color, FlexDirection, JustifyContent, Size, Style, TextStyle, UiRect, Val,
  },
  ui::PositionType,
};

pub(super) const SCOREBOARD_BACKGROUND: Color = Color::rgb(15. / 255., 15. / 255., 15. / 255.);
pub(super) const SCOREBOARD: Style = Style {
  flex_direction: FlexDirection::Column,
  align_items: AlignItems::Center,
  size: Size::new(Val::Percent(20.), Val::Auto),
  gap: Size::new(Val::Px(8.), Val::Px(8.)),
  padding: UiRect::all(Val::Px(10.)),
  ..Style::DEFAULT
};

pub(super) const SCORE_BACKGROUND: Color = Color::BLACK;
pub(super) const SCORE_HEIGHT: f32 = 40.;
pub(super) const SCORE: Style = Style {
  position_type: PositionType::Absolute,
  position: UiRect {
    left: Val::Undefined,
    right: Val::Undefined,
    top: Val::Px(0.),
    bottom: Val::Undefined,
  },
  justify_content: JustifyContent::SpaceBetween,
  align_items: AlignItems::Center,
  size: Size::new(Val::Percent(100.), Val::Auto),
  padding: UiRect::new(Val::Px(8.), Val::Px(8.), Val::Px(4.), Val::Px(4.)),
  ..Style::DEFAULT
};

pub(super) fn text(color: Color) -> TextStyle {
  TextStyle {
    font_size: 16.0,
    color,
    ..Default::default()
  }
}
