use fltk::enums::{Align, Color};
use fltk::frame::Frame;
use fltk::prelude::{WidgetExt, WindowExt};
use fltk::window::Window;
use pyo3::Python;
use tch::Device;

pub(crate) mod consts;
pub(crate) mod enums;
pub(crate) mod loss_fn;

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
