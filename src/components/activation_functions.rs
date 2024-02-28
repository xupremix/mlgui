#[derive(Debug, Clone)]
pub(crate) struct ActivationFunction;

impl ActivationFunction {
    pub(crate) fn new() -> Self {
        Self {}
    }
    pub(crate) fn connected(&self) -> bool {
        true
    }
}
