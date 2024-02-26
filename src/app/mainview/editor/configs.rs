use std::cell::RefCell;
use std::path::PathBuf;
use std::rc::Rc;

use fltk::app::Sender;
use fltk::button::Button;
use fltk::dialog::{FileDialogType, NativeFileChooser};
use fltk::enums::{Align, Color, Cursor, Event, Font, FrameType};
use fltk::frame::Frame;
use fltk::group::Group;
use fltk::menu::Choice;
use fltk::prelude::{GroupExt, WidgetBase};
use fltk::prelude::{MenuExt, WidgetExt};
use fltk::window::Window;
use pyo3::Python;
use tch::Device;
use tch::nn::Optimizer;
use tch::utils::has_vulkan;

use crate::utils::{
    AppEvent, BG_COLOR, check_mps_availability, CustomDialog, DEVICES, DRAG_THRESHOLD,
    HIGHLIGHT_COLOR, MENU_BAR_COLOR, MENU_BAR_RATIO,
};

pub(crate) struct ConfingList {
    pub(crate) window: Window,
    pub(crate) save_path: Rc<RefCell<Option<PathBuf>>>,
    pub(crate) device: Rc<RefCell<Option<Device>>>,
    pub(crate) optimizer: Rc<RefCell<Option<Optimizer>>>,
    pub(crate) loss_fn: Rc<RefCell<Option<String>>>,
    pub(crate) lr: Rc<RefCell<Option<f64>>>,
    pub(crate) batch_size: Rc<RefCell<Option<i64>>>,
    pub(crate) epochs: Rc<RefCell<Option<usize>>>,
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
        let save_path = Rc::new(RefCell::new(None));
        let (border, mut save_btn) = save_path_entry(save_path.clone(), p_w, p_h);

        // Device
        let device = Rc::new(RefCell::new(None));
        let (device_border, mut device_selector) = device_entry(device.clone(), &border, p_h);

        // Optimizer
        let optimizer: Rc<RefCell<Option<Optimizer>>> = Rc::new(RefCell::new(None));
        let optimizer_border = optimizer_entry(optimizer.clone(), &device_border, p_h);

        // Loss Function
        let loss_fn = Rc::new(RefCell::new(None));
        let loss_border = loss_entry(loss_fn.clone(), &optimizer_border, p_h);

        // Learning Rate
        let lr = Rc::new(RefCell::new(None));
        let lr_border = lr_entry(lr.clone(), &loss_border, p_h);

        // Batch Size
        let batch_size = Rc::new(RefCell::new(None));
        let batch_border = batch_entry(batch_size.clone(), &lr_border, p_h);

        // Epochs
        let epochs = Rc::new(RefCell::new(None));
        let epoch_border = epoch_entry(epochs.clone(), &batch_border, p_h);

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
                frame.resize(2, 1, window.w() - 2, frame.h());
                config_frame.resize(2, config_frame.y(), window.w() - 2, window.h());
                group.resize(2, group.y(), window.w(), window.h());
                // resize buttons
                save_btn.resize(
                    window.w() / 2 + 4,
                    save_btn.y(),
                    window.w() / 2 - 5,
                    save_btn.h(),
                );
                device_selector.resize(
                    save_btn.x(),
                    device_selector.y(),
                    save_btn.w(),
                    device_selector.h(),
                );
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
        Self {
            window,
            save_path,
            device,
            optimizer,
            loss_fn,
            lr,
            batch_size,
            epochs,
        }
    }
}

fn epoch_entry(epochs: Rc<RefCell<Option<usize>>>, batch_border: &Frame, p_h: i32) -> Frame {
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
    // Handle events
    epoch_border
}

fn batch_entry(batch_size: Rc<RefCell<Option<i64>>>, lr_border: &Frame, p_h: i32) -> Frame {
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
    // Handle events
    batch_border
}

fn lr_entry(lr: Rc<RefCell<Option<f64>>>, loss_border: &Frame, p_h: i32) -> Frame {
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
    // Handle events
    lr_border
}

fn loss_entry(loss: Rc<RefCell<Option<String>>>, device_border: &Frame, p_h: i32) -> Frame {
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
    // Handle events
    loss_border
}

fn optimizer_entry(
    optimizer: Rc<RefCell<Option<Optimizer>>>,
    device_border: &Frame,
    p_h: i32,
) -> Frame {
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
    // Handle events
    optimizer_border
}

fn device_entry(device: Rc<RefCell<Option<Device>>>, border: &Frame, p_h: i32) -> (Frame, Choice) {
    let mut device_border = Frame::default()
        .with_pos(border.x(), border.y() + p_h / MENU_BAR_RATIO)
        .with_size(border.w(), border.h());
    device_border.set_color(Color::White);
    device_border.set_frame(FrameType::FlatBox);
    let mut device_text = Frame::default()
        .with_pos(device_border.x(), device_border.y() + 1)
        .with_size(device_border.w() / 2, device_border.h() - 2)
        .with_label("Device: ")
        .with_align(Align::Inside | Align::Left);
    device_text.set_label_color(Color::White);
    device_text.set_frame(FrameType::FlatBox);
    device_text.set_color(BG_COLOR);
    // Handle events
    let mut device_selector = Choice::default()
        .with_pos(device_text.w() + 4, device_text.y())
        .with_size(device_text.w() - 5, device_border.h() - 2)
        .with_label("Select device:");
    device_selector.set_align(Align::Inside | Align::Center);
    device_selector.set_label_color(Color::White);
    device_selector.set_frame(FrameType::FlatBox);
    device_selector.set_color(BG_COLOR);
    device_selector.set_selection_color(HIGHLIGHT_COLOR);
    for device_name in DEVICES.iter() {
        if *device_name == "CUDA" {
            for device in 0..tch::Cuda::device_count() {
                let i = device_selector.add_choice(format!("CUDA({})", device).as_str());
                let mut entry = device_selector.at(i).unwrap();
                entry.set_label_color(Color::White);
            }
        } else {
            let i = device_selector.add_choice(device_name);
            let mut entry = device_selector.at(i).unwrap();
            entry.set_label_color(Color::White);
        }
    }
    device_selector.set_callback(move |selector| {
        let value = selector.value();
        selector.set_value(-1);
        device.replace(match value {
            0 => {
                selector.set_label(selector.at(value).unwrap().label().unwrap().as_str());
                Some(Device::Cpu)
            }
            1 => {
                if tch::Cuda::is_available() {
                    selector.set_label(selector.at(value).unwrap().label().unwrap().as_str());
                    Some(Device::Cuda(0))
                } else {
                    CustomDialog::show(
                        150,
                        40,
                        "Error",
                        "CUDA not available",
                        BG_COLOR,
                        Color::Red,
                    );
                    selector.set_label("Select device:");
                    None
                }
            }
            2 => Python::with_gil(|py| match check_mps_availability(py) {
                Ok(device) => {
                    selector.set_label(selector.at(value).unwrap().label().unwrap().as_str());
                    Some(device)
                }
                Err(e) => {
                    CustomDialog::show(400, 60, "Error", &e, BG_COLOR, Color::Red);
                    None
                }
            }),
            3 => {
                if has_vulkan() {
                    selector.set_label(selector.at(value).unwrap().label().unwrap().as_str());
                    Some(Device::Vulkan)
                } else {
                    CustomDialog::show(
                        200,
                        40,
                        "Error",
                        "Vulkan not available",
                        BG_COLOR,
                        Color::Red,
                    );
                    None
                }
            }
            _ => unreachable!(),
        });
    });
    (device_border, device_selector)
}

fn save_path_entry(save: Rc<RefCell<Option<PathBuf>>>, p_w: i32, p_h: i32) -> (Frame, Button) {
    let mut border = Frame::default()
        .with_pos(2, p_h / MENU_BAR_RATIO)
        .with_size(p_w, p_h / MENU_BAR_RATIO);
    border.set_color(Color::White);
    border.set_frame(FrameType::FlatBox);
    let mut save_path_text = Frame::default()
        .with_pos(border.x(), border.y() + 1)
        .with_size(border.w() / 2, border.h() - 2)
        .with_label("Model Save Path: ")
        .with_align(Align::Inside | Align::Left);
    save_path_text.set_label_color(Color::White);
    save_path_text.set_frame(FrameType::FlatBox);
    save_path_text.set_color(BG_COLOR);
    // Handle events
    let mut save_path_selector = Button::default()
        .with_pos(save_path_text.w() + 4, save_path_text.y())
        .with_size(save_path_text.w() - 5, border.h() - 2)
        .with_label("Select save path");
    save_path_selector.set_align(Align::Inside | Align::Center);
    save_path_selector.set_label_color(Color::White);
    save_path_selector.set_frame(FrameType::FlatBox);
    save_path_selector.set_color(BG_COLOR);
    save_path_selector.set_selection_color(HIGHLIGHT_COLOR);
    save_path_selector.set_callback(move |_| {
        let mut dialog = NativeFileChooser::new(FileDialogType::BrowseSaveFile);
        dialog.show();
        let filename = dialog.filename();
        if filename.ne(&PathBuf::new()) {
            let file = filename.with_extension("pt");
            let path = format!("Save path: {:?}", file);
            CustomDialog::show(
                path.len() as i32 * 8,
                40,
                "Success",
                path.as_str(),
                BG_COLOR,
                Color::Green,
            );
            save.replace(Some(file));
        } else {
            CustomDialog::show(150, 40, "Error", "No file selected", BG_COLOR, Color::Red);
        }
    });
    (border, save_path_selector)
}
