use std::path::PathBuf;
use fltk::enums::Event;
use fltk::enums::FrameType;
use fltk::frame::Frame;

use fltk::enums::Color;
use fltk::prelude::{GroupExt, WidgetBase, WidgetExt};
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
        let mut component = Frame::default()
            .with_size(100, 60)
            .center_of(&self.draw_area);
        component.set_frame(FrameType::FlatBox);
        component.set_color(Color::White);
        let mut inner_component = Frame::default()
            .with_size(98, 58)
            .center_of(&component)
            .with_label(*(layer.clone()));
        inner_component.set_label_color(Color::White);
        inner_component.set_frame(FrameType::FlatBox);
        inner_component.set_color(BG_COLOR);
        self.draw_area.add(&component);
        self.draw_area.add(&inner_component);
        self.draw_area.redraw();
        let mut set = false;
        inner_component.handle(move |component, event| {
            match event {
                Event::Push if !set => {
                    set = true;
                    true
                },
                Event::Drag if set => {
                    let (x, y) = (fltk::app::event_x(),fltk::app::event_y()) ;
                    component.resize(
                        component.x() - x,
                        component.y() - y,
                        component.w(),
                        component.h(),
                    );
                    eprintln!("X: {}, Y: {}", x, y);
                    true
                },
                _ => false,
            }
        });
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
