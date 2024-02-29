use std::path::PathBuf;

use fltk::enums::Color;
use fltk::prelude::{GroupExt, WidgetExt};
use fltk::window::Window;
use tch::Device;

use crate::components::activation_functions::ActivationFunctionType;
use crate::components::layers::LayerType;
use crate::components::NNComponent;
use crate::utils::consts::BG_COLOR;
use crate::utils::CustomDialog;
use crate::utils::loss_fn::LossFunction;

pub(crate) struct Playground {
    draw_area: Window,
    components: Vec<NNComponent>,
    first: Option<usize>,
}

impl Playground {
    pub(crate) fn new(p_w: i32, p_h: i32) -> Self {
        let mut draw_area = Window::default().with_size(p_w, p_h);
        draw_area.set_color(BG_COLOR);
        draw_area.end();

        Self {
            draw_area,
            components: vec![],
            first: None,
        }
    }

    pub(crate) fn add_layer(&mut self, layer: LayerType) {
        self.components.push(NNComponent::Layer {
            layer_type: layer,
            configured: false,
            observation_space: 0,
            action_space: 0,
            next: None,
        });
    }

    pub(crate) fn add_fn(&mut self, actv_fn: ActivationFunctionType) {
        self.components.push(NNComponent::ActivationFunction {
            fn_type: actv_fn,
            next: None,
        });
    }

    pub(crate) fn build_model(
        &self,
        save_path: PathBuf,
        device: Device,
        optimizer: String,
        loss_fn: LossFunction,
        lr: f64,
        batch_size: i64,
        epochs: usize,
    ) {
        CustomDialog::show(
            220,
            40,
            "Success",
            "Model built successfully",
            BG_COLOR,
            Color::Green,
        );
        eprintln!("Building model");
        eprintln!("Save path: {:?}", save_path);
        eprintln!("Device: {:?}", device);
        eprintln!("Optimizer: {}", optimizer);
        eprintln!("Loss function: {:?}", loss_fn);
        eprintln!("Learning rate: {}", lr);
        eprintln!("Batch size: {}", batch_size);
        eprintln!("Epochs: {}", epochs);
        eprintln!("Components: {:#?}", self.components);
    }
}
