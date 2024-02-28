use std::cell::RefCell;
use std::ffi::OsStr;
use std::path::PathBuf;
use std::rc::Rc;

use fltk::app::Sender;
use fltk::button::Button;
use fltk::dialog::{FileDialogType, NativeFileChooser};
use fltk::enums::{Align, Color, Cursor, Event, Font, FrameType};
use fltk::frame::Frame;
use fltk::group::Group;
use fltk::input::{FloatInput, IntInput};
use fltk::menu::Choice;
use fltk::prelude::{GroupExt, InputExt, WidgetBase};
use fltk::prelude::{MenuExt, WidgetExt};
use fltk::window::Window;
use pyo3::Python;
use tch::Device;
use tch::utils::has_vulkan;

use crate::utils::check_mps_availability;
use crate::utils::consts::{
    BG_COLOR, DEFAULT_BATCH_SIZE, DEFAULT_BATCH_SIZE_STR, DEFAULT_EPOCHS, DEFAULT_EPOCHS_STR,
    DEFAULT_LR, DEFAULT_LR_STR, DEVICES, DRAG_THRESHOLD, HIGHLIGHT_COLOR, LOSS_FUNCTIONS,
    MENU_BAR_COLOR, MENU_BAR_RATIO, OPTIMIZERS,
};
use crate::utils::CustomDialog;
use crate::utils::enums::AppEvent;
use crate::utils::loss_fn::{LossFunction, LossWidget};

pub(crate) struct ConfingList {
    pub(crate) window: Window,
    pub(crate) save_path: Rc<RefCell<Option<PathBuf>>>,
    pub(crate) device: Rc<RefCell<Option<Device>>>,
    pub(crate) optimizer: Rc<RefCell<Option<String>>>,
    pub(crate) loss_fn: Rc<RefCell<Option<LossFunction>>>,
    pub(crate) lr: Rc<RefCell<f64>>,
    pub(crate) batch_size: Rc<RefCell<i64>>,
    pub(crate) epochs: Rc<RefCell<usize>>,
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
        let optimizer: Rc<RefCell<Option<String>>> = Rc::new(RefCell::new(None));
        let (optimizer_border, mut optimizer_selector) =
            optimizer_entry(optimizer.clone(), &device_border, p_h);

        // Loss Function
        let loss_fn = Rc::new(RefCell::new(None));
        let (loss_border, mut loss_selector) = loss_entry(loss_fn.clone(), &optimizer_border, p_h);

        // Learning Rate
        let lr = Rc::new(RefCell::new(DEFAULT_LR));
        let (lr_border, mut lr_selector) = lr_entry(lr.clone(), &loss_border, p_h);

        // Batch Size
        let batch_size = Rc::new(RefCell::new(DEFAULT_BATCH_SIZE));
        let (batch_border, mut batch_selector) = batch_entry(batch_size.clone(), &lr_border, p_h);

        // Epochs
        let epochs = Rc::new(RefCell::new(DEFAULT_EPOCHS));
        let (epoch_border, mut epoch_selector) = epoch_entry(epochs.clone(), &batch_border, p_h);

        let mut build_btn_bg = Frame::default()
            .with_pos(p_w / 3, p_h - epoch_border.h())
            .with_size(p_w / 3, epoch_border.h());
        build_btn_bg.set_color(Color::White);
        build_btn_bg.set_frame(FrameType::FlatBox);
        let mut build_btn = Button::default()
            .with_size(build_btn_bg.w() - 4, build_btn_bg.h() - 4)
            .center_of(&build_btn_bg)
            .with_label("Build Model");
        build_btn.set_color(BG_COLOR);
        build_btn.set_frame(FrameType::FlatBox);
        build_btn.set_label_color(Color::White);

        let check_save_path = save_path.clone();
        let check_device = device.clone();
        let check_optimizer = optimizer.clone();
        let check_loss_fn = loss_fn.clone();

        build_btn.set_callback(move |_| {
            // check if all fields are filled and correct
            if check_save_path.borrow().is_none() {
                CustomDialog::show(
                    220,
                    40,
                    "Error",
                    "Save path not selected",
                    BG_COLOR,
                    Color::Red,
                );
            } else if check_device.borrow().is_none() {
                CustomDialog::show(
                    220,
                    40,
                    "Error",
                    "Device not selected",
                    BG_COLOR,
                    Color::Red,
                );
            } else if check_optimizer.borrow().is_none() {
                CustomDialog::show(
                    220,
                    40,
                    "Error",
                    "Optimizer not selected",
                    BG_COLOR,
                    Color::Red,
                );
            } else if check_loss_fn.borrow().is_none() {
                CustomDialog::show(
                    220,
                    40,
                    "Error",
                    "Loss function not selected",
                    BG_COLOR,
                    Color::Red,
                );
            } else {
                CustomDialog::show(
                    220,
                    40,
                    "Success",
                    "Model built successfully",
                    BG_COLOR,
                    Color::Green,
                );
            }
        });

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
                optimizer_selector.resize(
                    device_selector.x(),
                    optimizer_selector.y(),
                    device_selector.w(),
                    optimizer_selector.h(),
                );
                loss_selector.resize(
                    optimizer_selector.x(),
                    loss_selector.y(),
                    optimizer_selector.w(),
                    loss_selector.h(),
                );
                lr_selector.resize(
                    loss_selector.x(),
                    lr_selector.y(),
                    loss_selector.w(),
                    lr_selector.h(),
                );
                batch_selector.resize(
                    lr_selector.x(),
                    batch_selector.y(),
                    lr_selector.w(),
                    batch_selector.h(),
                );
                epoch_selector.resize(
                    batch_selector.x(),
                    epoch_selector.y(),
                    batch_selector.w(),
                    epoch_selector.h(),
                );
                build_btn.resize(
                    build_btn_bg.x() + 2,
                    build_btn_bg.y() + 2,
                    build_btn_bg.w() - 4,
                    build_btn_bg.h() - 4,
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

fn epoch_entry(epochs: Rc<RefCell<usize>>, batch_border: &Frame, p_h: i32) -> (Frame, IntInput) {
    let mut epoch_border = Frame::default()
        .with_pos(batch_border.x(), batch_border.y() + p_h / MENU_BAR_RATIO)
        .with_size(batch_border.w(), batch_border.h());
    epoch_border.set_color(Color::White);
    epoch_border.set_frame(FrameType::FlatBox);
    let mut epochs_text = Frame::default()
        .with_pos(epoch_border.x(), epoch_border.y() + 1)
        .with_size(epoch_border.w() / 2, epoch_border.h() - 2)
        .with_label("Epochs: ")
        .with_align(Align::Inside | Align::Left);
    epochs_text.set_label_color(Color::White);
    epochs_text.set_frame(FrameType::FlatBox);
    epochs_text.set_color(BG_COLOR);
    // Handle events
    let mut epochs_selector = IntInput::default()
        .with_pos(epochs_text.w() + 4, epochs_text.y())
        .with_size(epochs_text.w() - 5, epoch_border.h() - 2);
    epochs_selector.set_color(BG_COLOR);
    epochs_selector.set_frame(FrameType::FlatBox);
    epochs_selector.set_selection_color(HIGHLIGHT_COLOR);
    epochs_selector.set_value("1");
    epochs_selector.set_cursor_color(Color::White);
    epochs_selector.set_text_color(Color::White);
    epochs_selector.handle(move |input, event| match event {
        Event::KeyUp => {
            epochs.replace(match input.value().parse::<usize>() {
                Ok(v) if v > 0 => v,
                _ => {
                    CustomDialog::show(
                        200,
                        40,
                        "Error",
                        "Invalid epoch number",
                        BG_COLOR,
                        Color::Red,
                    );
                    input.set_value(DEFAULT_EPOCHS_STR);
                    DEFAULT_EPOCHS
                }
            });
            true
        }
        _ => false,
    });

    (epoch_border, epochs_selector)
}

fn batch_entry(batch_size: Rc<RefCell<i64>>, lr_border: &Frame, p_h: i32) -> (Frame, IntInput) {
    let mut batch_border = Frame::default()
        .with_pos(lr_border.x(), lr_border.y() + p_h / MENU_BAR_RATIO)
        .with_size(lr_border.w(), lr_border.h());
    batch_border.set_color(Color::White);
    batch_border.set_frame(FrameType::FlatBox);
    let mut batch_text = Frame::default()
        .with_pos(batch_border.x(), batch_border.y() + 1)
        .with_size(batch_border.w() / 2, batch_border.h() - 2)
        .with_label("Batch Size: ")
        .with_align(Align::Inside | Align::Left);
    batch_text.set_label_color(Color::White);
    batch_text.set_frame(FrameType::FlatBox);
    batch_text.set_color(BG_COLOR);
    // Handle events
    let mut batch_selector = IntInput::default()
        .with_pos(batch_text.w() + 4, batch_text.y())
        .with_size(batch_text.w() - 5, batch_border.h() - 2);
    batch_selector.set_color(BG_COLOR);
    batch_selector.set_frame(FrameType::FlatBox);
    batch_selector.set_selection_color(HIGHLIGHT_COLOR);
    batch_selector.set_value(DEFAULT_BATCH_SIZE_STR);
    batch_selector.set_cursor_color(Color::White);
    batch_selector.set_text_color(Color::White);
    batch_selector.handle(move |input, event| match event {
        Event::KeyUp => {
            batch_size.replace(match input.value().parse::<i64>() {
                Ok(v) if v > 0 => v,
                _ => {
                    CustomDialog::show(
                        200,
                        40,
                        "Error",
                        "Invalid batch size",
                        BG_COLOR,
                        Color::Red,
                    );
                    input.set_value(DEFAULT_BATCH_SIZE_STR);
                    DEFAULT_BATCH_SIZE
                }
            });
            true
        }
        _ => false,
    });

    (batch_border, batch_selector)
}

fn lr_entry(lr: Rc<RefCell<f64>>, loss_border: &Frame, p_h: i32) -> (Frame, FloatInput) {
    let mut lr_border = Frame::default()
        .with_pos(loss_border.x(), loss_border.y() + p_h / MENU_BAR_RATIO)
        .with_size(loss_border.w(), loss_border.h());
    lr_border.set_color(Color::White);
    lr_border.set_frame(FrameType::FlatBox);
    let mut lr_text = Frame::default()
        .with_pos(lr_border.x(), lr_border.y() + 1)
        .with_size(lr_border.w() / 2, lr_border.h() - 2)
        .with_label("Learning Rate: ")
        .with_align(Align::Inside | Align::Left);
    lr_text.set_label_color(Color::White);
    lr_text.set_frame(FrameType::FlatBox);
    lr_text.set_color(BG_COLOR);
    // Handle events
    let mut lr_selector = FloatInput::default()
        .with_pos(lr_text.w() + 4, lr_text.y())
        .with_size(lr_text.w() - 5, lr_border.h() - 2);
    lr_selector.set_color(BG_COLOR);
    lr_selector.set_frame(FrameType::FlatBox);
    lr_selector.set_selection_color(HIGHLIGHT_COLOR);
    lr_selector.set_value(DEFAULT_LR_STR);
    lr_selector.set_cursor_color(Color::White);
    lr_selector.set_text_color(Color::White);
    lr_selector.handle(move |input, event| match event {
        Event::KeyUp => {
            lr.replace(match input.value().parse::<f64>() {
                Ok(v) if v > 0.0 => v,
                _ => {
                    CustomDialog::show(200, 40, "Error", "Invalid lr", BG_COLOR, Color::Red);
                    input.set_value(DEFAULT_LR_STR);
                    DEFAULT_LR
                }
            });
            true
        }
        _ => false,
    });

    (lr_border, lr_selector)
}

fn loss_entry(
    loss: Rc<RefCell<Option<LossFunction>>>,
    device_border: &Frame,
    p_h: i32,
) -> (Frame, Choice) {
    let mut loss_border = Frame::default()
        .with_pos(device_border.x(), device_border.y() + p_h / MENU_BAR_RATIO)
        .with_size(device_border.w(), device_border.h());
    loss_border.set_color(Color::White);
    loss_border.set_frame(FrameType::FlatBox);
    let mut loss_text = Frame::default()
        .with_pos(loss_border.x(), loss_border.y() + 1)
        .with_size(loss_border.w() / 2, loss_border.h() - 2)
        .with_label("Loss Function: ")
        .with_align(Align::Inside | Align::Left);
    loss_text.set_label_color(Color::White);
    loss_text.set_frame(FrameType::FlatBox);
    loss_text.set_color(BG_COLOR);
    // Handle events
    let mut loss_selector = Choice::default()
        .with_pos(loss_text.w() + 4, loss_text.y())
        .with_size(loss_text.w() - 5, device_border.h() - 2)
        .with_label("Select loss fn:");
    loss_selector.set_align(Align::Inside | Align::Center);
    loss_selector.set_label_color(Color::White);
    loss_selector.set_frame(FrameType::FlatBox);
    loss_selector.set_color(BG_COLOR);
    loss_selector.set_selection_color(HIGHLIGHT_COLOR);
    for loss_name in LOSS_FUNCTIONS.iter() {
        let i = loss_selector.add_choice(loss_name);
        let mut entry = loss_selector.at(i).unwrap();
        entry.set_label_color(Color::White);
    }
    loss_selector.set_callback(move |selector| {
        let value = selector.value();
        let name = selector.at(value).unwrap().label().unwrap().to_string();
        selector.set_value(-1);
        loss.replace(None);
        LossWidget::show(value, &name, loss.clone());
        selector.set_label(match *loss.borrow() {
            None => "Select loss fn:",
            Some(_) => &name,
        });
    });
    (loss_border, loss_selector)
}

fn optimizer_entry(
    optimizer: Rc<RefCell<Option<String>>>,
    device_border: &Frame,
    p_h: i32,
) -> (Frame, Choice) {
    let mut optimizer_border = Frame::default()
        .with_pos(device_border.x(), device_border.y() + p_h / MENU_BAR_RATIO)
        .with_size(device_border.w(), device_border.h());
    optimizer_border.set_color(Color::White);
    optimizer_border.set_frame(FrameType::FlatBox);
    let mut optimizer_text = Frame::default()
        .with_pos(optimizer_border.x(), optimizer_border.y() + 1)
        .with_size(optimizer_border.w() / 2, optimizer_border.h() - 2)
        .with_label("Optimizer: ")
        .with_align(Align::Inside | Align::Left);
    optimizer_text.set_label_color(Color::White);
    optimizer_text.set_frame(FrameType::FlatBox);
    optimizer_text.set_color(BG_COLOR);
    // Handle events
    let mut optimizer_selector = Choice::default()
        .with_pos(optimizer_text.w() + 4, optimizer_text.y())
        .with_size(optimizer_text.w() - 5, device_border.h() - 2)
        .with_label("Select optimizer:");
    optimizer_selector.set_align(Align::Inside | Align::Center);
    optimizer_selector.set_label_color(Color::White);
    optimizer_selector.set_frame(FrameType::FlatBox);
    optimizer_selector.set_color(BG_COLOR);
    optimizer_selector.set_selection_color(HIGHLIGHT_COLOR);
    for optimizer_name in OPTIMIZERS.iter() {
        let i = optimizer_selector.add_choice(optimizer_name);
        let mut entry = optimizer_selector.at(i).unwrap();
        entry.set_label_color(Color::White);
    }
    optimizer_selector.set_callback(move |selector| {
        let value = selector.value();
        selector.set_value(-1);
        let name = selector.at(value).unwrap().label().unwrap().to_string();
        selector.set_label(name.as_str());
        optimizer.replace(Some(name));
    });
    (optimizer_border, optimizer_selector)
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
                        200,
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
    save_path_selector.set_callback(move |selector| {
        let mut dialog = NativeFileChooser::new(FileDialogType::BrowseSaveFile);
        dialog.show();
        let filename = dialog.filename();
        if filename.ne(&PathBuf::new()) {
            if let Some(ext) = filename.extension() {
                if ext.ne(OsStr::new("pt")) {
                    CustomDialog::show(
                        300,
                        40,
                        "Error",
                        "Extension must be either empty or .pt",
                        BG_COLOR,
                        Color::Red,
                    );
                } else {
                    let file = filename.with_extension("pt");
                    selector.set_label(file.file_name().unwrap().to_str().unwrap());
                    save.replace(Some(file));
                }
            } else {
                let file = filename.with_extension("pt");
                selector.set_label(file.file_name().unwrap().to_str().unwrap());
                save.replace(Some(file));
            };
        } else {
            CustomDialog::show(200, 40, "Error", "No file selected", BG_COLOR, Color::Red);
        }
    });
    (border, save_path_selector)
}
