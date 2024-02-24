use fltk::enums::Color;

pub(crate) const WINDOW_WIDTH: i32 = 1080;
pub(crate) const WINDOW_HEIGHT: i32 = 720;
pub(crate) const WINDOW_TITLE: &str = "Linear GUI";
pub(crate) const MENU_BAR_HEIGHT: i32 = 30;
pub(crate) const MENU_BAR_COLOR: Color = Color::from_hex(0x21252B);
pub(crate) const BG_COLOR: Color = Color::from_hex(0x282C34);

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
