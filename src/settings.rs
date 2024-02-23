use fltk::enums::Color;

pub(crate) const WINDOW_WIDTH: i32 = 1080;
pub(crate) const WINDOW_HEIGHT: i32 = 720;
pub(crate) const WINDOW_TITLE: &str = "Linear GUI";
pub(crate) const MENU_BAR_HEIGHT: i32 = 30;
pub(crate) const MENU_BAR_COLOR: Color = Color::from_hex(0x21252B);
pub(crate) const BG_COLOR: Color = Color::from_hex(0x282C34);

#[derive(Debug, Clone, Copy)]
pub(crate) enum AppEvent {
    Editor,
    Training,
    Settings,
    Help,
}

#[derive(Debug, Clone, Eq, PartialEq, Copy)]
pub(crate) enum AppMode {
    Editor,
    Training,
}
