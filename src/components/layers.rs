use std::fmt::Debug;
use std::ops::Deref;
use std::str::FromStr;

use strum::EnumIter;
use tch::nn::{BatchNormConfig, ConvConfig, ConvTransposeConfig, LinearConfig, RNNConfig};

#[derive(Debug, Clone, EnumIter)]
pub(crate) enum LayerType {
    Linear(LinearConfig),
    Lstm(RNNConfig),
    Gru(RNNConfig),
    BatchNorm1D(BatchNormConfig),
    BatchNorm2D(BatchNormConfig),
    BatchNorm3D(BatchNormConfig),
    Conv1D(ConvConfig),
    Conv2D(ConvConfig),
    Conv3D(ConvConfig),
    ConvTranspose1D(ConvTransposeConfig),
    ConvTranspose2D(ConvTransposeConfig),
    ConvTranspose3D(ConvTransposeConfig),
}

impl FromStr for LayerType {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Linear" => Ok(LayerType::Linear(LinearConfig::default())),
            "Lstm" => Ok(LayerType::Lstm(RNNConfig::default())),
            "Gru" => Ok(LayerType::Gru(RNNConfig::default())),
            "BatchNorm1D" => Ok(LayerType::BatchNorm1D(BatchNormConfig::default())),
            "BatchNorm2D" => Ok(LayerType::BatchNorm2D(BatchNormConfig::default())),
            "BatchNorm3D" => Ok(LayerType::BatchNorm3D(BatchNormConfig::default())),
            "Conv1D" => Ok(LayerType::Conv1D(ConvConfig::default())),
            "Conv2D" => Ok(LayerType::Conv2D(ConvConfig::default())),
            "Conv3D" => Ok(LayerType::Conv3D(ConvConfig::default())),
            "ConvTranspose1D" => Ok(LayerType::ConvTranspose1D(ConvTransposeConfig::default())),
            "ConvTranspose2D" => Ok(LayerType::ConvTranspose2D(ConvTransposeConfig::default())),
            "ConvTranspose3D" => Ok(LayerType::ConvTranspose3D(ConvTransposeConfig::default())),
            _ => Err("Invalid LayerType"),
        }
    }
}

impl Deref for LayerType {
    type Target = &'static str;

    fn deref(&self) -> &Self::Target {
        match self {
            LayerType::Linear(_) => &"Linear",
            LayerType::Lstm(_) => &"Lstm",
            LayerType::Gru(_) => &"Gru",
            LayerType::BatchNorm1D(_) => &"BatchNorm1D",
            LayerType::BatchNorm2D(_) => &"BatchNorm2D",
            LayerType::BatchNorm3D(_) => &"BatchNorm3D",
            LayerType::Conv1D(_) => &"Conv1D",
            LayerType::Conv2D(_) => &"Conv2D",
            LayerType::Conv3D(_) => &"Conv3D",
            LayerType::ConvTranspose1D(_) => &"ConvTranspose1D",
            LayerType::ConvTranspose2D(_) => &"ConvTranspose2D",
            LayerType::ConvTranspose3D(_) => &"ConvTranspose3D",
        }
    }
}
