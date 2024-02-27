use std::cell::RefCell;
use std::rc::Rc;

use fltk::button::ToggleButton;
use fltk::enums::{Align, Color, Event, FrameType};
use fltk::frame::Frame;
use fltk::input::{FloatInput, Input, IntInput};
use fltk::menu::Choice;
use fltk::prelude::{ButtonExt, GroupExt, InputExt, MenuExt, WidgetBase, WidgetExt, WindowExt};
use fltk::window::Window;
use pyo3::Python;
use tch::{Device, Reduction};

pub(crate) const WINDOW_TITLE: &str = "Ml Gui";
pub(crate) const MENU_BAR_RATIO: i32 = 24;
pub(crate) const COMPONENT_LIST_RATIO: i32 = 5;
pub(crate) const CONFIG_LIST_RATIO: i32 = 4;
pub(crate) const DRAG_THRESHOLD: i32 = 4;
pub(crate) const MENU_BAR_COLOR: Color = Color::from_hex(0x21252B);
pub(crate) const BG_COLOR: Color = Color::from_hex(0x282C34);
pub(crate) const HIGHLIGHT_COLOR: Color = Color::from_hex(0x3E4452);
pub(crate) const DEFAULT_LR: f64 = 0.01;
pub(crate) const DEFAULT_BATCH_SIZE: i64 = 20;
pub(crate) const DEFAULT_EPOCHS: usize = 100;

#[derive(Debug, Clone)]
pub(crate) enum AppEvent {
    Editor,
    Training,
    Settings,
    Help,
    AddLayer(String),
    AddActivationFunction(String),
}

#[derive(Debug, Clone, Eq, PartialEq, Copy)]
pub(crate) enum AppMode {
    Editor,
    Training,
}

pub(crate) const LAYERS: [&str; 13] = [
    "Linear",
    "LSTM",
    "GRU",
    "BatchNorm1D",
    "BatchNorm2D",
    "BatchNorm3D",
    "Conv",
    "Conv1D",
    "Conv2D",
    "Conv3D",
    "ConvTranspose1D",
    "ConvTranspose2D",
    "ConvTranspose3D",
];

pub(crate) const ACTIVATION_FUNCTIONS: [&str; 5] =
    ["ReLU", "Leaky ReLU", "Softmax", "Sigmoid", "Tanh"];

pub(crate) const DEVICES: [&str; 4] = ["CPU", "CUDA", "MPS", "VULKAN"];
pub(crate) const OPTIMIZERS: [&str; 4] = ["SGD", "Adam", "AdamW", "RMSprop"];
pub(crate) const LOSS_FUNCTIONS: [&str; 7] =
    ["MSE", "CrossEntropy", "BCE", "NLL", "CTC", "Huber", "L1"];
pub(crate) struct CustomDialog {}
pub(crate) struct LossWidget {}

impl CustomDialog {
    pub(crate) fn show(
        width: i32,
        height: i32,
        title: &str,
        message: &str,
        bg_color: Color,
        message_color: Color,
    ) {
        let mut window = Window::default()
            .with_size(width, height)
            .with_label(title)
            .center_screen();
        let mut f = Frame::default()
            .with_label(message)
            .with_align(Align::Center)
            .center_of_parent();
        window.set_color(bg_color);
        f.set_label_color(message_color);
        window.show();
    }
}

pub(crate) fn check_mps_availability(py: Python) -> Result<Device, String> {
    let torch = py
        .import("torch")
        .map_err(|e| format!("Error importing torch: \n{:?}", e))?;
    let backends = torch
        .getattr("backends")
        .map_err(|e| format!("Error accessing the backends module: \n{:?}", e))?;
    let mps = backends
        .getattr("mps")
        .map_err(|e| format!("Error accessing the mps module: \n{:?}", e))?;
    let is_available_fn = mps
        .getattr("is_available")
        .map_err(|e| format!("Could not find the is_available_fn: \n{:?}", e))?;
    let ris = is_available_fn
        .call0()
        .map_err(|e| format!("Error with the is_available_fn: \n{:?}", e))?;
    if ris
        .extract()
        .map_err(|e| format!("Error extracting the is_available_fn result: \n{:?}", e))?
    {
        Ok(Device::Mps)
    } else {
        Err("MPS is not available on this system".to_string())
    }
}

const LOSS_WINDOW_WIDTH: i32 = 400;
const LOSS_WINDOW_HEIGHT: i32 = 400;
impl LossWidget {
    pub(crate) fn show(loss_fn_i: i32, title: &str, loss: Rc<RefCell<Option<LossFunction>>>) {
        let mut window = Window::default()
            .with_size(LOSS_WINDOW_WIDTH, LOSS_WINDOW_HEIGHT)
            .with_label(title)
            .center_screen();
        window.set_label_color(Color::White);
        window.set_color(BG_COLOR);
        window.set_frame(FrameType::FlatBox);
        match LOSS_FUNCTIONS[loss_fn_i as usize] {
            "MSE" => {
                window.set_size(LOSS_WINDOW_WIDTH, LOSS_WINDOW_HEIGHT / 4);
                // reduction parameter
                let reduction = Self::reduction_parameter();

                // show the window
                window.end();
                window.show();
                window.set_callback(move |window| {
                    window.hide();
                    let reduction = match reduction.label().as_str() {
                        "Select reduction" => {
                            CustomDialog::show(
                                200,
                                40,
                                "Error",
                                "Please select a reduction",
                                BG_COLOR,
                                Color::Red,
                            );
                            return;
                        }
                        "Sum" => Reduction::Sum,
                        "Mean" => Reduction::Mean,
                        _ => unreachable!(),
                    };
                    loss.replace(Some(LossFunction::Mse { reduction }));
                });
            }
            "CrossEntropy" => {
                window.set_size(LOSS_WINDOW_WIDTH, LOSS_WINDOW_HEIGHT / 2);
                // reduction parameter
                let reduction = Self::reduction_parameter();
                // smoothing parameter
                let smoothing = Self::smoothing_delta_parameter("Smoothing: ");

                // show the window
                window.end();
                window.show();
                window.set_callback(move |window| {
                    window.hide();
                    let reduction = match reduction.label().as_str() {
                        "Select reduction" => {
                            CustomDialog::show(
                                200,
                                40,
                                "Error",
                                "Please select a reduction",
                                BG_COLOR,
                                Color::Red,
                            );
                            return;
                        }
                        "Sum" => Reduction::Sum,
                        "Mean" => Reduction::Mean,
                        _ => unreachable!(),
                    };
                    let smoothing = match smoothing.value().parse::<f64>() {
                        Ok(value) => value,
                        Err(e) => {
                            CustomDialog::show(
                                350,
                                60,
                                "Error",
                                &format!("Error parsing smoothing: \n{}", e),
                                BG_COLOR,
                                Color::Red,
                            );
                            return;
                        }
                    };
                    loss.replace(Some(LossFunction::CrossEntropy {
                        reduction,
                        smoothing,
                    }));
                });
            }
            "BCE" => {
                window.set_size(LOSS_WINDOW_WIDTH, LOSS_WINDOW_HEIGHT / 4);
                // reduction parameter
                let reduction = Self::reduction_parameter();

                // show the window
                window.end();
                window.show();
                window.set_callback(move |window| {
                    window.hide();
                    let reduction = match reduction.label().as_str() {
                        "Select reduction" => {
                            CustomDialog::show(
                                200,
                                40,
                                "Error",
                                "Please select a reduction",
                                BG_COLOR,
                                Color::Red,
                            );
                            return;
                        }
                        "Sum" => Reduction::Sum,
                        "Mean" => Reduction::Mean,
                        _ => unreachable!(),
                    };
                    loss.replace(Some(LossFunction::Bce { reduction }));
                });
            }
            "NLL" => {
                // no parameters
                loss.replace(Some(LossFunction::Nll));
            }
            "CTC" => {
                window.set_size(LOSS_WINDOW_WIDTH, LOSS_WINDOW_HEIGHT / 4 * 3);
                // reduction parameter
                let reduction = Self::reduction_parameter();
                // blank parameter
                let blank = Self::blank_parameter();
                // zero_infinity parameter
                let zero_infinity = Self::zero_infinity_parameter();

                // show the window
                window.end();
                window.show();
                window.set_callback(move |window| {
                    window.hide();
                    let reduction = match reduction.label().as_str() {
                        "Select reduction" => {
                            CustomDialog::show(
                                200,
                                40,
                                "Error",
                                "Please select a reduction",
                                BG_COLOR,
                                Color::Red,
                            );
                            return;
                        }
                        "Sum" => Reduction::Sum,
                        "Mean" => Reduction::Mean,
                        _ => unreachable!(),
                    };
                    let blank = match blank.value().parse::<i64>() {
                        Ok(value) => value,
                        Err(e) => {
                            CustomDialog::show(
                                350,
                                60,
                                "Error",
                                &format!("Error parsing blank: \n{}", e),
                                BG_COLOR,
                                Color::Red,
                            );
                            return;
                        }
                    };
                    loss.replace(Some(LossFunction::Ctc {
                        reduction,
                        blank,
                        zero_infinity: zero_infinity.is_set(),
                    }));
                });
            }
            "Huber" => {
                window.set_size(LOSS_WINDOW_WIDTH, LOSS_WINDOW_HEIGHT / 2);
                // reduction parameter
                let reduction = Self::reduction_parameter();
                // delta parameter
                let delta = Self::smoothing_delta_parameter("Delta: ");

                // show the window
                window.end();
                window.show();
                window.set_callback(move |window| {
                    window.hide();
                    let delta = match delta.value().parse::<f64>() {
                        Ok(value) => value,
                        Err(e) => {
                            CustomDialog::show(
                                350,
                                60,
                                "Error",
                                &format!("Error parsing delta: \n{}", e),
                                BG_COLOR,
                                Color::Red,
                            );
                            return;
                        }
                    };
                    let reduction = match reduction.label().as_str() {
                        "Select reduction" => {
                            CustomDialog::show(
                                200,
                                40,
                                "Error",
                                "Please select a reduction",
                                BG_COLOR,
                                Color::Red,
                            );
                            return;
                        }
                        "Sum" => Reduction::Sum,
                        "Mean" => Reduction::Mean,
                        _ => unreachable!(),
                    };
                    loss.replace(Some(LossFunction::Huber { reduction, delta }));
                });
            }
            "L1" => {
                window.set_size(LOSS_WINDOW_WIDTH, LOSS_WINDOW_HEIGHT / 4);
                // reduction parameter
                let reduction = Self::reduction_parameter();
                // show the window
                window.end();
                window.show();
                window.set_callback(move |window| {
                    window.hide();
                    match reduction.label().as_str() {
                        "Select reduction" => {
                            CustomDialog::show(
                                200,
                                40,
                                "Error",
                                "Please select a reduction",
                                BG_COLOR,
                                Color::Red,
                            );
                        }
                        "Sum" => {
                            loss.replace(Some(LossFunction::L1 {
                                reduction: Reduction::Sum,
                            }));
                        }
                        "Mean" => {
                            loss.replace(Some(LossFunction::L1 {
                                reduction: Reduction::Mean,
                            }));
                        }
                        _ => unreachable!(),
                    };
                });
            }
            _ => unreachable!(),
        }
        while window.shown() {
            fltk::app::wait();
        }
    }
    fn reduction_parameter() -> Choice {
        let mut border = Frame::default()
            .with_pos(0, 0)
            .with_size(LOSS_WINDOW_WIDTH, LOSS_WINDOW_HEIGHT / 4);
        border.set_color(Color::White);
        border.set_frame(FrameType::FlatBox);
        let mut text = Frame::default()
            .with_pos(border.x() + 2, border.y() + 2)
            .with_size(border.w() / 2 - 4, border.h() - 4)
            .with_label("Reduction: ");
        text.set_label_color(Color::White);
        text.set_frame(FrameType::FlatBox);
        text.set_color(BG_COLOR);

        let mut choice = Choice::default()
            .with_pos(LOSS_WINDOW_WIDTH / 2 + 2, text.y())
            .with_size(text.w(), text.h())
            .with_label("Select reduction");
        choice.set_color(BG_COLOR);
        choice.set_selection_color(HIGHLIGHT_COLOR);
        choice.set_align(Align::Center);
        choice.set_label_color(Color::White);
        choice.set_frame(FrameType::FlatBox);

        let i = choice.add_choice("Sum");
        let mut item = choice.at(i).unwrap();
        item.set_label_color(Color::White);
        let i = choice.add_choice("Mean");
        let mut item = choice.at(i).unwrap();
        item.set_label_color(Color::White);

        choice.set_callback(move |selector| {
            selector.set_label(&selector.text(selector.value()).unwrap());
            selector.set_value(-1);
        });
        choice
    }
    fn smoothing_delta_parameter(name: &str) -> FloatInput {
        let mut border = Frame::default()
            .with_pos(0, LOSS_WINDOW_HEIGHT / 4)
            .with_size(LOSS_WINDOW_WIDTH, LOSS_WINDOW_HEIGHT / 4);
        border.set_color(Color::White);
        border.set_frame(FrameType::FlatBox);
        let mut text = Frame::default()
            .with_pos(border.x() + 2, border.y() + 2)
            .with_size(border.w() / 2 - 4, border.h() - 4)
            .with_label(name);
        text.set_label_color(Color::White);
        text.set_frame(FrameType::FlatBox);
        text.set_color(BG_COLOR);

        let mut input = FloatInput::default()
            .with_pos(LOSS_WINDOW_WIDTH / 2 + 2, text.y())
            .with_size(text.w(), text.h());
        input.set_color(BG_COLOR);
        input.set_frame(FrameType::FlatBox);
        input.set_selection_color(HIGHLIGHT_COLOR);
        input.set_value("0.0");
        input.set_cursor_color(Color::White);
        input.set_text_color(Color::White);

        input
    }
    fn blank_parameter() -> IntInput {
        let mut border = Frame::default()
            .with_pos(0, LOSS_WINDOW_HEIGHT / 4)
            .with_size(LOSS_WINDOW_WIDTH, LOSS_WINDOW_HEIGHT / 4);
        border.set_color(Color::White);
        border.set_frame(FrameType::FlatBox);
        let mut text = Frame::default()
            .with_pos(border.x() + 2, border.y() + 2)
            .with_size(border.w() / 2 - 4, border.h() - 4)
            .with_label("Blank: ");
        text.set_label_color(Color::White);
        text.set_frame(FrameType::FlatBox);
        text.set_color(BG_COLOR);

        let mut input = IntInput::default()
            .with_pos(LOSS_WINDOW_WIDTH / 2 + 2, text.y())
            .with_size(text.w(), text.h());
        input.set_color(BG_COLOR);
        input.set_frame(FrameType::FlatBox);
        input.set_selection_color(HIGHLIGHT_COLOR);
        input.set_value("0");
        input.set_cursor_color(Color::White);
        input.set_text_color(Color::White);

        input
    }
    fn zero_infinity_parameter() -> ToggleButton {
        let mut border = Frame::default()
            .with_pos(0, LOSS_WINDOW_HEIGHT / 2)
            .with_size(LOSS_WINDOW_WIDTH, LOSS_WINDOW_HEIGHT / 4);
        border.set_color(Color::White);
        border.set_frame(FrameType::FlatBox);
        let mut text = Frame::default()
            .with_pos(border.x() + 2, border.y() + 2)
            .with_size(border.w() / 2 - 4, border.h() - 4)
            .with_label("Zero - Infinity: ");
        text.set_label_color(Color::White);
        text.set_frame(FrameType::FlatBox);
        text.set_color(BG_COLOR);

        let mut input = ToggleButton::default()
            .with_pos(LOSS_WINDOW_WIDTH / 2 + 2, text.y())
            .with_size(text.w(), text.h())
            .with_label("✖️");

        input.set_color(BG_COLOR);
        input.set_frame(FrameType::FlatBox);
        input.set_selection_color(HIGHLIGHT_COLOR);

        input.set_callback(move |btn| {
            btn.set_label(if btn.is_set() { "✔️" } else { "✖️" });
        });

        input
    }
    fn weight_parameter(i: i32) -> Input {
        let mut border = Frame::default()
            .with_pos(0, i * LOSS_WINDOW_HEIGHT / 4)
            .with_size(LOSS_WINDOW_WIDTH, LOSS_WINDOW_HEIGHT / 4);
        border.set_color(Color::White);
        border.set_frame(FrameType::FlatBox);
        let mut text = Frame::default()
            .with_pos(border.x() + 2, border.y() + 2)
            .with_size(border.w() / 2 - 4, border.h() - 4)
            .with_label("Weights: ");
        text.set_label_color(Color::White);
        text.set_frame(FrameType::FlatBox);
        text.set_color(BG_COLOR);

        let mut input = Input::default()
            .with_pos(LOSS_WINDOW_WIDTH / 2 + 2, text.y())
            .with_size(text.w(), text.h());
        input.set_value("Eg: 0.1, 0.9, 0.0");
        input.set_align(Align::Inside | Align::Center);
        input.set_color(BG_COLOR);
        input.set_frame(FrameType::FlatBox);
        input.set_cursor_color(Color::White);
        input.set_selection_color(HIGHLIGHT_COLOR);
        input.set_text_color(Color::White);

        input.handle(move |input, evt| {
            let mut set = false;
            match evt {
                Event::Push if !set => {
                    set = true;
                    input.set_value("");
                    true
                }
                _ => false,
            }
        });
        input
    }
}

#[derive(Debug, Clone)]
pub(crate) enum LossFunction {
    Mse {
        reduction: Reduction,
    },
    CrossEntropy {
        reduction: Reduction,
        smoothing: f64,
    },
    Bce {
        reduction: Reduction,
    },
    Nll,
    Ctc {
        reduction: Reduction,
        blank: i64,
        zero_infinity: bool,
    },
    Huber {
        reduction: Reduction,
        delta: f64,
    },
    L1 {
        reduction: Reduction,
    },
}
