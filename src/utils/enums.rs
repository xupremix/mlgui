use std::ops::Deref;

use crate::components::NNComponent;

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

#[derive(Debug, Clone)]
pub(crate) enum LayerType {
    Linear,
    Lstm,
    Gru,
    BatchNorm1D,
    BatchNorm2D,
    BatchNorm3D,
    Conv,
    Conv1D,
    Conv2D,
    Conv3D,
    ConvTranspose1D,
    ConvTranspose2D,
    ConvTranspose3D,
}

pub(crate) enum ActivationFunctionType {
    ReLU,
    LeakyReLU,
    Softmax,
    Sigmoid,
    Tanh,
}

impl Deref for LayerType {
    type Target = &'static str;

    fn deref(&self) -> &Self::Target {
        match self {
            LayerType::Linear => &"Linear",
            LayerType::Lstm => &"LSTM",
            LayerType::Gru => &"GRU",
            LayerType::BatchNorm1D => &"BatchNorm1D",
            LayerType::BatchNorm2D => &"BatchNorm2D",
            LayerType::BatchNorm3D => &"BatchNorm3D",
            LayerType::Conv => &"Conv",
            LayerType::Conv1D => &"Conv1D",
            LayerType::Conv2D => &"Conv2D",
            LayerType::Conv3D => &"Conv3D",
            LayerType::ConvTranspose1D => &"ConvTranspose1D",
            LayerType::ConvTranspose2D => &"ConvTranspose2D",
            LayerType::ConvTranspose3D => &"ConvTranspose3D",
        }
    }
}

impl From<&str> for NNComponent {
    fn from(value: &str) -> Self {
        match value {
            "LayerType::Linear" => NNComponent::Layer(crate::components::layers::Layer::new()),
            "LayerType::Lstm" => NNComponent::Layer(crate::components::layers::Layer::new()),
            "LayerType::Gru" => NNComponent::Layer(crate::components::layers::Layer::new()),
            "LayerType::BatchNorm1D" => NNComponent::Layer(crate::components::layers::Layer::new()),
            "LayerType::BatchNorm2D" => NNComponent::Layer(crate::components::layers::Layer::new()),
            "LayerType::BatchNorm3D" => NNComponent::Layer(crate::components::layers::Layer::new()),
            "LayerType::Conv" => NNComponent::Layer(crate::components::layers::Layer::new()),
            "LayerType::Conv1D" => NNComponent::Layer(crate::components::layers::Layer::new()),
            "LayerType::Conv2D" => NNComponent::Layer(crate::components::layers::Layer::new()),
            "LayerType::Conv3D" => NNComponent::Layer(crate::components::layers::Layer::new()),
            "LayerType::ConvTranspose1D" => {
                NNComponent::Layer(crate::components::layers::Layer::new())
            }
            "LayerType::ConvTranspose2D" => {
                NNComponent::Layer(crate::components::layers::Layer::new())
            }
            "LayerType::ConvTranspose3D" => {
                NNComponent::Layer(crate::components::layers::Layer::new())
            }
            "ActivationFunctionType::ReLU" => NNComponent::ActivationFunction(
                crate::components::activation_functions::ActivationFunction::new(),
            ),
            "ActivationFunctionType::LeakyReLU" => NNComponent::ActivationFunction(
                crate::components::activation_functions::ActivationFunction::new(),
            ),
            "ActivationFunctionType::Softmax" => NNComponent::ActivationFunction(
                crate::components::activation_functions::ActivationFunction::new(),
            ),
            "ActivationFunctionType::Sigmoid" => NNComponent::ActivationFunction(
                crate::components::activation_functions::ActivationFunction::new(),
            ),
            "ActivationFunctionType::Tanh" => NNComponent::ActivationFunction(
                crate::components::activation_functions::ActivationFunction::new(),
            ),
            _ => panic!("Invalid component type"),
        }
    }
}
