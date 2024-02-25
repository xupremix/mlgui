use fltk::app::Sender;
use fltk::enums::{Align, Color, Cursor, Event, Font, FrameType};
use fltk::frame::Frame;
use fltk::group::Group;
use fltk::prelude::{GroupExt, WidgetBase};
use fltk::prelude::WidgetExt;
use fltk::window::Window;

use crate::utils::{AppEvent, BG_COLOR, DRAG_THRESHOLD, MENU_BAR_COLOR, MENU_BAR_RATIO};

pub(crate) struct ConfingList {
    pub(crate) window: Window,
}

fltk::widget_extends!(ConfingList, Window, window);

impl ConfingList {
    pub(crate) fn new(
        evt_sender: Sender<AppEvent>,
        p_x: i32,
        p_y: i32,
        mut p_w: i32,
        p_h: i32,
    ) -> Self {
        let mut window = Window::default().with_pos(p_x, p_y).with_size(p_w, p_h);
        window.set_color(Color::White);

        let mut custom_frame_border = Frame::default()
            .with_pos(2, 0)
            .with_size(p_w, p_h / MENU_BAR_RATIO);
        custom_frame_border.set_frame(FrameType::FlatBox);
        custom_frame_border.set_color(Color::White);

        let mut frame = Frame::default()
            .with_pos(2, 1)
            .with_size(p_w - 2, p_h / MENU_BAR_RATIO - 2)
            .with_label("Configs");
        frame.set_frame(FrameType::FlatBox);
        frame.set_label_font(Font::HelveticaBold);
        frame.set_color(MENU_BAR_COLOR);
        frame.set_label_color(Color::White);

        let mut config_frame = Frame::default()
            .with_pos(2, p_h / MENU_BAR_RATIO)
            .with_size(p_w, p_h);
        config_frame.set_frame(FrameType::FlatBox);
        config_frame.set_color(BG_COLOR);

        let mut group = Group::default()
            .with_pos(2, p_h / MENU_BAR_RATIO)
            .with_size(p_w, p_h);

        // Save Path
        let mut border = Frame::default()
            .with_pos(2, p_h / MENU_BAR_RATIO)
            .with_size(p_w, p_h / MENU_BAR_RATIO);
        border.set_color(Color::White);
        border.set_frame(FrameType::FlatBox);
        let mut save_path_text = Frame::default()
            .with_pos(border.x(), border.y() + 1)
            .with_size(border.w(), border.h() - 2)
            .with_label("Model Save Path: ")
            .with_align(Align::Inside | Align::Left);
        save_path_text.set_label_color(Color::White);
        save_path_text.set_frame(FrameType::FlatBox);
        save_path_text.set_color(BG_COLOR);


        // Device
        let mut device_border = Frame::default()
            .with_pos(border.x(), border.y() + p_h / MENU_BAR_RATIO)
            .with_size(border.w(), border.h());
        device_border.set_color(Color::White);
        device_border.set_frame(FrameType::FlatBox);
        let mut device_text = Frame::default()
            .with_pos(device_border.x(), device_border.y() + 1)
            .with_size(device_border.w(), device_border.h() - 2)
            .with_label("Device: ")
            .with_align(Align::Inside | Align::Left);
        device_text.set_label_color(Color::White);
        device_text.set_frame(FrameType::FlatBox);
        device_text.set_color(BG_COLOR);

        // Optimizer
        let mut optimizer_border = Frame::default()
            .with_pos(device_border.x(), device_border.y() + p_h / MENU_BAR_RATIO)
            .with_size(device_border.w(), device_border.h());
        optimizer_border.set_color(Color::White);
        optimizer_border.set_frame(FrameType::FlatBox);
        let mut optimizer_text = Frame::default()
            .with_pos(optimizer_border.x(), optimizer_border.y() + 1)
            .with_size(optimizer_border.w(), optimizer_border.h() - 2)
            .with_label("Optimizer: ")
            .with_align(Align::Inside | Align::Left);
        optimizer_text.set_label_color(Color::White);
        optimizer_text.set_frame(FrameType::FlatBox);
        optimizer_text.set_color(BG_COLOR);

        // Loss Function
        let mut loss_border = Frame::default()
            .with_pos(device_border.x(), device_border.y() + p_h / MENU_BAR_RATIO)
            .with_size(device_border.w(), device_border.h());
        loss_border.set_color(Color::White);
        loss_border.set_frame(FrameType::FlatBox);
        let mut loss_text = Frame::default()
            .with_pos(loss_border.x(), loss_border.y() + 1)
            .with_size(loss_border.w(), loss_border.h() - 2)
            .with_label("Loss Function: ")
            .with_align(Align::Inside | Align::Left);
        loss_text.set_label_color(Color::White);
        loss_text.set_frame(FrameType::FlatBox);
        loss_text.set_color(BG_COLOR);

        // Learning Rate
        let mut lr_border = Frame::default()
            .with_pos(loss_border.x(), loss_border.y() + p_h / MENU_BAR_RATIO)
            .with_size(loss_border.w(), loss_border.h());
        lr_border.set_color(Color::White);
        lr_border.set_frame(FrameType::FlatBox);
        let mut lr_text = Frame::default()
            .with_pos(lr_border.x(), lr_border.y() + 1)
            .with_size(lr_border.w(), lr_border.h() - 2)
            .with_label("Learning Rate: ")
            .with_align(Align::Inside | Align::Left);
        lr_text.set_label_color(Color::White);
        lr_text.set_frame(FrameType::FlatBox);
        lr_text.set_color(BG_COLOR);

        // Batch Size
        let mut batch_border = Frame::default()
            .with_pos(lr_border.x(), lr_border.y() + p_h / MENU_BAR_RATIO)
            .with_size(lr_border.w(), lr_border.h());
        batch_border.set_color(Color::White);
        batch_border.set_frame(FrameType::FlatBox);
        let mut batch_text = Frame::default()
            .with_pos(batch_border.x(), batch_border.y() + 1)
            .with_size(batch_border.w(), batch_border.h() - 2)
            .with_label("Batch Size: ")
            .with_align(Align::Inside | Align::Left);
        batch_text.set_label_color(Color::White);
        batch_text.set_frame(FrameType::FlatBox);
        batch_text.set_color(BG_COLOR);

        // Epochs
        let mut epoch_border = Frame::default()
            .with_pos(batch_border.x(), batch_border.y() + p_h / MENU_BAR_RATIO)
            .with_size(batch_border.w(), batch_border.h());
        epoch_border.set_color(Color::White);
        epoch_border.set_frame(FrameType::FlatBox);
        let mut epochs_text = Frame::default()
            .with_pos(epoch_border.x(), epoch_border.y() + 1)
            .with_size(epoch_border.w(), epoch_border.h() - 2)
            .with_label("Epochs: ")
            .with_align(Align::Inside | Align::Left);
        epochs_text.set_label_color(Color::White);
        epochs_text.set_frame(FrameType::FlatBox);
        epochs_text.set_color(BG_COLOR);

        group.end();
        window.end();

        let mut enabled = false;
        window.handle(move |window, event| match event {
            Event::Push => {
                enabled = fltk::app::event_x() < DRAG_THRESHOLD;
                true
            }
            Event::Drag if enabled => {
                let x = fltk::app::event_x();
                p_w -= x;
                window.resize(window.x() + x, window.y(), p_w, p_h);
                frame.resize(2, 1, window.w() - 2, p_h / MENU_BAR_RATIO - 2);
                config_frame.resize(2, p_h / MENU_BAR_RATIO, window.w() - 2, window.h());
                group.resize(2, p_h / MENU_BAR_RATIO, window.w() - 2, window.h());
                true
            }
            Event::Move => {
                let x = fltk::app::event_x();
                if x < DRAG_THRESHOLD {
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
