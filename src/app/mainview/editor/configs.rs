use fltk::enums::{Color, Cursor, Event, Font, FrameType};
use fltk::frame::Frame;
use fltk::prelude::{GroupExt, WidgetBase};
use fltk::prelude::WidgetExt;
use fltk::window::Window;

use crate::settings::{BG_COLOR, MENU_BAR_COLOR, MENU_BAR_HEIGHT, WINDOW_HEIGHT, WINDOW_WIDTH};

pub(crate) struct ConfingList {
    pub(crate) window: Window,
}

fltk::widget_extends!(ConfingList, Window, window);

impl ConfingList {
    pub(crate) fn new() -> Self {
        let mut width = 205;
        let mut window = Window::new(
            WINDOW_WIDTH - width,
            0,
            width,
            WINDOW_HEIGHT - MENU_BAR_HEIGHT,
            None,
        );
        window.set_color(Color::White);

        let mut custom_frame_border = Frame::new(2, 0, 203, 30, None);
        custom_frame_border.set_frame(FrameType::FlatBox);
        custom_frame_border.set_color(Color::White);

        let mut frame = Frame::new(2, 1, width - 2, 28, "Configs");
        frame.set_frame(FrameType::FlatBox);
        frame.set_label_font(Font::HelveticaBold);
        frame.set_color(MENU_BAR_COLOR);
        frame.set_label_color(Color::White);

        let mut config_frame = Frame::new(
            2,
            30,
            WINDOW_WIDTH - width,
            WINDOW_HEIGHT - MENU_BAR_HEIGHT - 30,
            None,
        );
        config_frame.set_frame(FrameType::FlatBox);
        config_frame.set_color(BG_COLOR);

        window.end();

        let mut enabled = false;
        window.handle(move |window, event| match event {
            Event::Push => {
                enabled = fltk::app::event_x() < 4;
                true
            }
            Event::Drag => {
                let x = fltk::app::event_x();
                if enabled {
                    width -= x;
                    window.resize(
                        window.x() + x,
                        window.y(),
                        width,
                        WINDOW_HEIGHT - MENU_BAR_HEIGHT,
                    );
                    frame.resize(2, 1, window.w() - 2, 28);
                    config_frame.resize(2, 30, window.w() - 2, window.h());
                }
                true
            }
            Event::Move => {
                let x = fltk::app::event_x();
                if x < 4 {
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
        Self { window }
    }
}
