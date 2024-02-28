#[derive(Debug, Clone)]
pub struct Layer;

impl Layer {
    pub(crate) fn new() -> Self {
        Self {}
    }
    pub(crate) fn connected(&self) -> bool {
        true
    }
}
