use fltk::enums::Color;

pub(crate) const BASE_COMPONENT_HEIGHT: i32 = 45;
pub(crate) const BASE_COMPONENT_WIDTH: i32 = 100;
pub(crate) const DEVICES: [&str; 4] = ["CPU", "CUDA", "MPS", "VULKAN"];
pub(crate) const OPTIMIZERS: [&str; 4] = ["SGD", "Adam", "AdamW", "RMSprop"];
pub(crate) const LOSS_FUNCTIONS: [&str; 7] =
    ["MSE", "CrossEntropy", "BCE", "NLL", "CTC", "Huber", "L1"];
pub(crate) const LOSS_WINDOW_WIDTH: i32 = 400;
pub(crate) const LOSS_WINDOW_HEIGHT: i32 = 400;
pub(crate) const WINDOW_TITLE: &str = "Ml Gui";
pub(crate) const MENU_BAR_RATIO: i32 = 24;
pub(crate) const COMPONENT_LIST_RATIO: i32 = 5;
pub(crate) const CONFIG_LIST_RATIO: i32 = 4;
pub(crate) const DRAG_THRESHOLD: i32 = 4;
pub(crate) const MENU_BAR_COLOR: Color = Color::from_hex(0x21252B);
pub(crate) const BG_COLOR: Color = Color::from_hex(0x282C34);
pub(crate) const HIGHLIGHT_COLOR: Color = Color::from_hex(0x3E4452);
pub(crate) const DEFAULT_LR: f64 = 0.01;
pub(crate) const DEFAULT_LR_STR: &str = "0.01";
pub(crate) const DEFAULT_BATCH_SIZE: i64 = 20;
pub(crate) const DEFAULT_BATCH_SIZE_STR: &str = "20";
pub(crate) const DEFAULT_EPOCHS: usize = 100;
pub(crate) const DEFAULT_EPOCHS_STR: &str = "100";
