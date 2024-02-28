use fltk::app::Sender;
use fltk::button::Button;
use fltk::prelude::{GroupExt, WidgetBase, WidgetExt};
use fltk::window::{DoubleWindow, Window};

use crate::app::mainview::editor::component_list::ComponentList;
use crate::app::mainview::editor::configs::ConfingList;
use crate::app::mainview::editor::playground::Playground;
use crate::utils::consts::{BG_COLOR, COMPONENT_LIST_RATIO, CONFIG_LIST_RATIO};
use crate::utils::enums::AppEvent;

mod component_list;
mod configs;
mod playground;

pub(crate) struct EditorView {
    window: DoubleWindow,
}

fltk::widget_extends!(EditorView, Window, window);

impl EditorView {
    pub(crate) fn new(evt_sender: Sender<AppEvent>, p_w: i32, p_h: i32) -> Self {
        let mut window = Window::default().with_size(p_w, p_h);
        window.set_color(BG_COLOR);

        // playground
        Playground::new(p_w, p_h);

        // graph display

        // component list
        ComponentList::new(evt_sender.clone(), p_w / COMPONENT_LIST_RATIO, p_h);

        // configs
        ConfingList::new(
            evt_sender,
            p_w - p_w / CONFIG_LIST_RATIO,
            0,
            p_w / CONFIG_LIST_RATIO,
            p_h,
        );

        let btn = Button::new(0, 0, 80, 40, "Editor").center_of(&window);
        window.end();
        Self { window }
    }
}
