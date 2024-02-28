use fltk::prelude::{GroupExt, WidgetExt};
use fltk::window::Window;

use crate::components::NNComponent;

pub(crate) struct Playground {
    components: Vec<NNComponent>,
}

impl Playground {
    pub(crate) fn new(p_w: i32, p_h: i32) -> Self {
        let draw_area = Window::default().with_size(p_w, p_h);
        draw_area.end();
        let out = Self { components: vec![] };
        out.add_layer("LayerType::Linear");
        out.add_activation_function();
        out
    }
    fn add_layer(&self, layer: &str) {
        let t: NNComponent = "LayerType::Linear".into();
        eprintln!("{:?}", t);
    }
    fn add_activation_function(&self) {
        let t: NNComponent = "ActivationFunctionType::ReLU".into();
        eprintln!("{:?}", t);
    }
}
