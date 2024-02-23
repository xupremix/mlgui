use std::cell::RefCell;

use fltk::app::Sender;
use fltk::button::Button;
use fltk::enums::{Color, Cursor, Event, FrameType};
use fltk::group::Group;
use fltk::prelude::{GroupExt, WidgetBase, WidgetExt};
use fltk::window::Window;

use crate::settings::{AppEvent, BG_COLOR, MENU_BAR_HEIGHT, WINDOW_HEIGHT};

pub(crate) struct ComponentList {
    pub(crate) window: Window,
    pub(crate) evt_sender: Sender<AppEvent>,
}

fltk::widget_extends!(ComponentList, Window, window);

impl ComponentList {
    pub(crate) fn new(evt_sender: Sender<AppEvent>) -> Self {
        let mut window = Window::new(0, 0, 205, WINDOW_HEIGHT - MENU_BAR_HEIGHT, None);
        window.set_color(Color::White);

        let mut group = Group::default().with_size(203, WINDOW_HEIGHT - MENU_BAR_HEIGHT);
        group.set_frame(FrameType::FlatBox);
        group.set_color(BG_COLOR);
        let mut btn = Button::new(0, 0, 80, 40, "Editor").center_of(&group);
        window.end();

        let enabled = RefCell::new(false);
        let threshold = RefCell::new(200);

        window.handle(move |window, event| match event {
            Event::Push => {
                let coords = fltk::app::event_coords();
                enabled.replace(coords.0 + window.x() > *threshold.borrow());
                true
            }
            Event::Drag => {
                let coords = fltk::app::event_coords();
                if *enabled.borrow() {
                    threshold.replace(coords.0 - 4);
                    window.resize(0, 0, coords.0 + window.x(), WINDOW_HEIGHT - MENU_BAR_HEIGHT);
                    group.resize(0, 0, coords.0 - 2, WINDOW_HEIGHT - MENU_BAR_HEIGHT);
                    let diff = coords.0 - window.x();
                    if diff < 40 {
                        btn.hide();
                    } else {
                        btn.show();
                    }
                }
                true
            }
            Event::Move => {
                let coords = fltk::app::event_coords();
                if coords.0 > *threshold.borrow() + window.x() {
                    fltk::draw::set_cursor(Cursor::E);
                } else {
                    fltk::draw::set_cursor(Cursor::Default);
                }
                true
            }
            Event::Leave => {
                fltk::draw::set_cursor(Cursor::Default);
                true
            }
            _ => false,
        });
        Self { window, evt_sender }
    }
}
