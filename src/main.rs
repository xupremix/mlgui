use crate::app::Application;

mod app;
mod components;
mod settings;

fn main() {
    let app = Application::new();
    app.run();
}
