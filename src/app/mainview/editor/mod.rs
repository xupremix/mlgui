use fltk::app::Sender;
use fltk::button::Button;
use fltk::prelude::{GroupExt, WidgetBase, WidgetExt};
use fltk::window::{DoubleWindow, Window};

use crate::app::mainview::editor::component_list::ComponentList;
use crate::settings::{AppEvent, BG_COLOR, MENU_BAR_HEIGHT, WINDOW_HEIGHT, WINDOW_WIDTH};

mod component_list;
mod configs;
mod playground;

pub(crate) struct EditorView {
    window: DoubleWindow,
    evt_sender: Sender<AppEvent>,
}

fltk::widget_extends!(EditorView, Window, window);

impl EditorView {
    pub(crate) fn new(evt_sender: Sender<AppEvent>) -> Self {
        let mut window = Window::new(0, 0, WINDOW_WIDTH, WINDOW_HEIGHT - MENU_BAR_HEIGHT, None);
        window.set_color(BG_COLOR);
        // component list
        ComponentList::new(evt_sender);
        // graph display
        // playground
        // configs
        let btn = Button::new(0, 0, 80, 40, "Editor").center_of(&window);
        window.end();
        Self { window, evt_sender }
    }
}
