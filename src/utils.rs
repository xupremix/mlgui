use fltk::enums::{Align, Color};
use fltk::frame::Frame;
use fltk::prelude::{WidgetExt, WindowExt};
use fltk::window::Window;
use pyo3::Python;
use tch::Device;

pub(crate) const WINDOW_TITLE: &str = "Mlgui";
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

pub(crate) struct CustomDialog {}

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
        while window.shown() {
            fltk::app::wait();
        }
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
    let is_available: bool = ris
        .extract()
        .map_err(|e| format!("Error extracting the is_available_fn result: \n{:?}", e))?;
    if is_available {
        Ok(Device::Mps)
    } else {
        Err("MPS is not available on this system".to_string())
    }
}
