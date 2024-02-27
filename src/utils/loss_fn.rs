use std::cell::RefCell;
use std::rc::Rc;

use fltk::button::ToggleButton;
use fltk::enums::{Align, Color, FrameType};
use fltk::frame::Frame;
use fltk::input::{FloatInput, IntInput};
use fltk::menu::Choice;
use fltk::prelude::{ButtonExt, GroupExt, InputExt, MenuExt, WidgetExt, WindowExt};
use fltk::window::Window;
use tch::Reduction;

use crate::utils::consts::{BG_COLOR, HIGHLIGHT_COLOR, LOSS_WINDOW_HEIGHT, LOSS_WINDOW_WIDTH};
use crate::utils::CustomDialog;

pub(crate) struct LossWidget {}
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
impl LossWidget {
    pub(crate) fn show(loss_fn_i: i32, title: &str, loss: Rc<RefCell<Option<LossFunction>>>) {
        let mut window = Window::default()
            .with_size(LOSS_WINDOW_WIDTH, LOSS_WINDOW_HEIGHT)
            .with_label(title)
            .center_screen();
        window.set_label_color(Color::White);
        window.set_color(BG_COLOR);
        window.set_frame(FrameType::FlatBox);
        match loss_fn_i {
            0 => {
                // MSE
                window.set_size(LOSS_WINDOW_WIDTH, LOSS_WINDOW_HEIGHT / 4);
                let reduction = Self::reduction_parameter();
                // show the window
                window.end();
                window.show();
                window.set_callback(move |window| {
                    window.hide();
                    if let Some(reduction) = get_reduction_value(&reduction) {
                        loss.replace(Some(LossFunction::Mse { reduction }));
                    }
                })
            }
            1 => {
                // CrossEntropy
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
                    if let (Some(reduction), Some(smoothing)) = (
                        get_reduction_value(&reduction),
                        get_smoothing_delta_value(&smoothing),
                    ) {
                        loss.replace(Some(LossFunction::CrossEntropy {
                            reduction,
                            smoothing,
                        }));
                    }
                });
            }
            2 => {
                // BCE
                window.set_size(LOSS_WINDOW_WIDTH, LOSS_WINDOW_HEIGHT / 4);
                // reduction parameter
                let reduction = Self::reduction_parameter();
                // show the window
                window.end();
                window.show();
                window.set_callback(move |window| {
                    window.hide();
                    if let Some(reduction) = get_reduction_value(&reduction) {
                        loss.replace(Some(LossFunction::Bce { reduction }));
                    }
                })
            }
            3 => {
                // NLL
                loss.replace(Some(LossFunction::Nll));
            }
            4 => {
                // CTC
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
                    if let (Some(reduction), Some(blank)) =
                        (get_reduction_value(&reduction), get_blank_value(&blank))
                    {
                        loss.replace(Some(LossFunction::Ctc {
                            reduction,
                            blank,
                            zero_infinity: zero_infinity.is_set(),
                        }));
                    }
                });
            }
            5 => {
                // Huber
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
                    if let (Some(reduction), Some(delta)) = (
                        get_reduction_value(&reduction),
                        get_smoothing_delta_value(&delta),
                    ) {
                        loss.replace(Some(LossFunction::Huber { reduction, delta }));
                    }
                });
            }
            _ => {
                // L1
                window.set_size(LOSS_WINDOW_WIDTH, LOSS_WINDOW_HEIGHT / 4);
                // reduction parameter
                let reduction = Self::reduction_parameter();
                // show the window
                window.end();
                window.show();
                window.set_callback(move |window| {
                    window.hide();
                    if let Some(reduction) = get_reduction_value(&reduction) {
                        loss.replace(Some(LossFunction::L1 { reduction }));
                    }
                });
            }
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
}

fn get_reduction_value(selector: &Choice) -> Option<Reduction> {
    match selector.label().as_str() {
        "Sum" => Some(Reduction::Sum),
        "Mean" => Some(Reduction::Mean),
        _ => {
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
    }
}

fn get_smoothing_delta_value(input: &FloatInput) -> Option<f64> {
    match input.value().parse::<f64>() {
        Ok(value) => Some(value),
        Err(e) => {
            CustomDialog::show(
                350,
                60,
                "Error",
                &format!("Error parsing value: \n{}", e),
                BG_COLOR,
                Color::Red,
            );
            None
        }
    }
}

fn get_blank_value(input: &IntInput) -> Option<i64> {
    match input.value().parse::<i64>() {
        Ok(value) => Some(value),
        Err(e) => {
            CustomDialog::show(
                350,
                60,
                "Error",
                &format!("Error parsing value: \n{}", e),
                BG_COLOR,
                Color::Red,
            );
            None
        }
    }
}
