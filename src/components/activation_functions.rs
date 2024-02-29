use std::ops::Deref;
use std::str::FromStr;

use strum::EnumIter;
use tch::Kind::Float;
use tch::Tensor;

#[derive(Debug, Clone, Copy, Eq, PartialEq, EnumIter)]
pub(crate) enum ActivationFunctionType {
    ReLU,
    Sigmoid,
    Tanh,
    Softmax,
    LeakyReLU,
    Flatten,
}

impl FromStr for ActivationFunctionType {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "ReLU" => Ok(ActivationFunctionType::ReLU),
            "Sigmoid" => Ok(ActivationFunctionType::Sigmoid),
            "Tanh" => Ok(ActivationFunctionType::Tanh),
            "Softmax" => Ok(ActivationFunctionType::Softmax),
            "LeakyReLU" => Ok(ActivationFunctionType::LeakyReLU),
            "Flatten" => Ok(ActivationFunctionType::Flatten),
            _ => Err("Invalid ActivationFunctionType"),
        }
    }
}

impl Deref for ActivationFunctionType {
    type Target = &'static str;

    fn deref(&self) -> &Self::Target {
        match self {
            ActivationFunctionType::ReLU => &"ReLU",
            ActivationFunctionType::Sigmoid => &"Sigmoid",
            ActivationFunctionType::Tanh => &"Tanh",
            ActivationFunctionType::Softmax => &"Softmax",
            ActivationFunctionType::LeakyReLU => &"LeakyReLU",
            ActivationFunctionType::Flatten => &"Flatten",
        }
    }
}

impl ActivationFunctionType {
    pub(crate) fn get(&self) -> impl Fn(&Tensor) -> Tensor {
        match self {
            ActivationFunctionType::ReLU => Tensor::relu,
            ActivationFunctionType::Sigmoid => Tensor::sigmoid,
            ActivationFunctionType::Tanh => Tensor::tanh,
            ActivationFunctionType::Softmax => |x: &Tensor| x.softmax(-1, Float),
            ActivationFunctionType::LeakyReLU => Tensor::leaky_relu,
            ActivationFunctionType::Flatten => |x: &Tensor| x.view([-1, x.size()[1]]),
        }
    }
}
