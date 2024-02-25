use fltk::app::Sender;
use fltk::button::Button;
use fltk::prelude::{GroupExt, WidgetBase, WidgetExt};
use fltk::window::{DoubleWindow, Window};

use crate::utils::{AppEvent, BG_COLOR};

pub(crate) struct TrainingView {
    window: DoubleWindow,
    evt_sender: Sender<AppEvent>,
}

fltk::widget_extends!(TrainingView, Window, window);

impl TrainingView {
    pub(crate) fn new(evt_sender: Sender<AppEvent>, p_w: i32, p_h: i32) -> Self {
        let mut window = Window::default().with_size(p_w, p_h);
        window.set_color(BG_COLOR);

        let btn = Button::new(0, 0, 80, 40, "Training").center_of(&window);
        window.end();
        Self { window, evt_sender }
    }
}
