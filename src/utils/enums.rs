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
