use crate::components::activation_functions::ActivationFunctionType;
use crate::components::layers::LayerType;

pub(crate) mod activation_functions;
pub(crate) mod layers;

// head of the network is being kept by the playground
#[derive(Debug)]
pub(crate) enum NNComponent {
    Layer {
        layer_type: LayerType,
        configured: bool,
        observation_space: i64,
        action_space: i64,
        next: Option<usize>,
    },
    ActivationFunction {
        fn_type: ActivationFunctionType,
        next: Option<usize>,
    },
}
