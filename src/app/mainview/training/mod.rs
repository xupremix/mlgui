use fltk::app::Sender;
use fltk::button::Button;
use fltk::prelude::{GroupExt, WidgetBase, WidgetExt};
use fltk::window::{DoubleWindow, Window};

use crate::settings::{AppEvent, BG_COLOR, MENU_BAR_HEIGHT, WINDOW_HEIGHT, WINDOW_WIDTH};

pub(crate) struct TrainingView {
    window: DoubleWindow,
    evt_sender: Sender<AppEvent>,
}

fltk::widget_extends!(TrainingView, Window, window);

impl TrainingView {
    pub(crate) fn new(evt_sender: Sender<AppEvent>) -> Self {
        let mut window = Window::new(0, 0, WINDOW_WIDTH, WINDOW_HEIGHT - MENU_BAR_HEIGHT, None);
        window.set_color(BG_COLOR);
        let btn = Button::new(0, 0, 80, 40, "Training").center_of(&window);
        window.end();
        Self { window, evt_sender }
    }
}
