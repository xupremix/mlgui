use fltk::enums::{Align, Color, Event, FrameType};
use fltk::frame::Frame;
use fltk::menu::Choice;
use fltk::prelude::{GroupExt, MenuExt, WidgetBase, WidgetExt, WindowExt};
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
    pub(crate) fn show(loss_fn_i: i32) -> Option<LossFunction> {
        let mut window = Window::default()
            .with_size(LOSS_WINDOW_WIDTH, LOSS_WINDOW_HEIGHT)
            .center_screen();
        window.set_color(BG_COLOR);
        window.set_frame(FrameType::FlatBox);
        match LOSS_FUNCTIONS[loss_fn_i as usize] {
            "MSE" => {
                window.set_size(LOSS_WINDOW_WIDTH, LOSS_WINDOW_HEIGHT / 2);
                // reduction parameter
                let reduction = Self::reduction_parameter();
                // confirm button
                let confirm = Self::confirm(1);

                // show the window
                window.end();
                window.show();
            }
            "CrossEntropy" => {
                // reduction parameter
                Self::reduction_parameter();
                // smoothing parameter
                Self::smoothing_parameter();
                // weight tensor
                Self::weight_parameter();
                // confirm button
                Self::confirm(3);

                // show the window
                window.end();
                window.show();
            }
            "BCE" => {
                window.set_size(LOSS_WINDOW_WIDTH, LOSS_WINDOW_HEIGHT / 4 * 3);
                // reduction parameter
                Self::reduction_parameter();
                // weight tensor
                Self::weight_parameter();
                // confirm button
                Self::confirm(2);

                // show the window
                window.end();
                window.show();
            }
            "NLL" => {
                // no parameters
            }
            "CTC" => {
                // reduction parameter
                Self::reduction_parameter();
                // blank parameter
                Self::blank_parameter();
                // zero_infinity parameter
                Self::zero_infinity_parameter();
                // confirm button
                Self::confirm(3);

                // show the window
                window.end();
                window.show();
            }
            "Huber" => {
                window.set_size(LOSS_WINDOW_WIDTH, LOSS_WINDOW_HEIGHT / 4 * 3);
                // reduction parameter
                Self::reduction_parameter();
                // delta parameter
                Self::delta_parameter();
                // confirm button
                Self::confirm(2);

                // show the window
                window.end();
                window.show();
            }
            "L1" => {
                window.set_size(LOSS_WINDOW_WIDTH, LOSS_WINDOW_HEIGHT / 2);
                // reduction parameter
                let reduction = Self::reduction_parameter();
                // confirm button
                let confirm = Self::confirm(1);

                // show the window
                window.end();
                window.show();
                window.handle(move |window, evt| match evt {
                    Event::Push => {
                        if fltk::app::event_inside(
                            confirm.x(),
                            confirm.y(),
                            confirm.w(),
                            confirm.h(),
                        ) {
                            let loss = match reduction.label().as_str() {
                                "Select reduction" => {
                                    CustomDialog::show(
                                        200,
                                        40,
                                        "Error",
                                        "Please select a reduction",
                                        BG_COLOR,
                                        Color::Red,
                                    );
                                    None
                                }
                                "Sum" => Some(LossFunction::L1 {
                                    reduction: Reduction::Sum,
                                }),
                                "Mean" => Some(LossFunction::L1 {
                                    reduction: Reduction::Mean,
                                }),
                                _ => unreachable!(),
                            };
                            window.hide();
                            // loss
                        }
                        true
                    }
                    _ => false,
                })
            }
            _ => unreachable!(),
        }
        Some(LossFunction::Nll)
        // None
    }
    fn confirm(i: i32) -> Frame {
        let mut border = Frame::default()
            .with_pos(LOSS_WINDOW_WIDTH / 3, i * LOSS_WINDOW_HEIGHT / 4)
            .with_size(LOSS_WINDOW_WIDTH / 3, LOSS_WINDOW_HEIGHT / 4);
        border.set_color(Color::White);
        border.set_frame(FrameType::FlatBox);
        let mut out = Frame::default()
            .with_pos(border.x() + 1, border.y() + 1)
            .with_size(border.w() - 2, border.h() - 2)
            .with_label("Confirm");
        out.set_label_color(Color::White);
        out.set_frame(FrameType::BorderBox);
        out.set_color(BG_COLOR);
        out
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
    fn smoothing_parameter() {}
    fn blank_parameter() {}
    fn zero_infinity_parameter() {}
    fn weight_parameter() {}
    fn delta_parameter() {}
}
#[derive(Debug, Clone)]
pub(crate) enum LossFunction {
    Mse {
        reduction: Reduction,
    },
    CrossEntropy {
        reduction: Reduction,
        smoothing: f64,
        weight: Option<Vec<f64>>,
    },
    Bce {
        reduction: Reduction,
        weight: Option<Vec<f64>>,
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
