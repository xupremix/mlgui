use crate::components::activation_functions::ActivationFunction;
use crate::components::layers::Layer;

pub(crate) mod activation_functions;
pub(crate) mod layers;

#[derive(Debug, Clone)]
pub(crate) enum NNComponent {
    Layer(Layer),
    ActivationFunction(ActivationFunction),
}

impl NNComponent {
    fn connected(&self) -> bool {
        match self {
            NNComponent::Layer(l) => l.connected(),
            NNComponent::ActivationFunction(a) => a.connected(),
        }
    }
}
