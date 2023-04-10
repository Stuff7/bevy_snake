use bevy::prelude::{
  AlignItems, AssetServer, Color, FlexDirection, JustifyContent, Res, Size, Style, TextStyle,
  UiRect, Val,
};

pub(super) const SCORE_BOARD_BACKGROUND: Color = Color::rgb(15. / 255., 15. / 255., 15. / 255.);
pub(super) const SCORE_BOARD: Style = Style {
  flex_direction: FlexDirection::Column,
  align_items: AlignItems::Center,
  size: Size::new(Val::Percent(20.), Val::Auto),
  gap: Size::new(Val::Px(8.), Val::Px(8.)),
  padding: UiRect::all(Val::Px(10.)),
  ..Style::DEFAULT
};

pub(super) const PLACE_BACKGROUND: Color = Color::BLACK;
pub(super) const PLACE: Style = Style {
  justify_content: JustifyContent::SpaceBetween,
  align_items: AlignItems::Center,
  size: Size::new(Val::Percent(100.), Val::Auto),
  padding: UiRect::new(Val::Px(8.), Val::Px(8.), Val::Px(4.), Val::Px(4.)),
  ..Style::DEFAULT
};

pub(super) fn text(asset_server: &Res<AssetServer>) -> TextStyle {
  TextStyle {
    font: asset_server.load("fonts/UbuntuMono-Regular.ttf"),
    font_size: 16.0,
    color: Color::default(),
  }
}
