use crate::app::Application;

mod app;
pub(crate) mod components;
mod settings;

fn main() {
    let app = Application::new();
    app.run();
}
