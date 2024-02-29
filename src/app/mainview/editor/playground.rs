use std::cell::RefCell;
use std::path::PathBuf;
use std::rc::Rc;
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
use crate::utils::consts::{BASE_COMPONENT_HEIGHT, BASE_COMPONENT_WIDTH, BG_COLOR};
use crate::utils::CustomDialog;
use crate::utils::loss_fn::LossFunction;

pub(crate) struct Playground {
    draw_area: Rc<RefCell<Window>>,
    components: Vec<NNComponent>,
    first: Option<usize>,
}

impl Playground {
    pub(crate) fn new(p_w: i32, p_h: i32) -> Self {
        let mut draw_area = Window::default().with_size(p_w, p_h);
        draw_area.set_color(BG_COLOR);
        draw_area.end();

        Self {
            draw_area: Rc::new(RefCell::new(draw_area)),
            components: vec![],
            first: None,
        }
    }

    pub(crate) fn add_layer(&mut self, layer: LayerType) {
        let draw_area = self.draw_area.clone();
        let label = *layer;
        let mut component = Frame::default()
            .with_size(BASE_COMPONENT_WIDTH.max(label.len() as i32 * 11), BASE_COMPONENT_HEIGHT)
            .center_of(&*draw_area.borrow());
        component.set_frame(FrameType::FlatBox);
        component.set_color(Color::White);
        let mut inner_component = Frame::default()
            .with_size(component.w() - 4, component.h() - 4)
            .center_of(&component)
            .with_label(*layer);
        inner_component.set_label_color(Color::White);
        inner_component.set_frame(FrameType::FlatBox);
        inner_component.set_color(BG_COLOR);
        draw_area.borrow_mut().add(&component);
        draw_area.borrow_mut().add(&inner_component);
        draw_area.borrow_mut().redraw();
        let mut set = false;
        let mut prev = (-1, -1);
        inner_component.handle(move |inner_component, event| {
            match event {
                Event::Push => {
                    if !set {
                        set = true;
                    }
                    prev = fltk::app::event_coords();
                    true
                },
                Event::Drag if set => {
                    let new_coords = fltk::app::event_coords();
                    let (new_x, new_y) = (
                        component.x() + (new_coords.0 - prev.0),
                        component.y() + (new_coords.1 - prev.1),
                    );
                    prev = new_coords;
                    component.set_pos(new_x, new_y);
                    inner_component.set_pos(new_x + 2, new_y + 2);
                    draw_area.borrow_mut().redraw();
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
