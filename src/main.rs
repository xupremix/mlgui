use crate::app::Application;

mod app;
pub(crate) mod components;
pub(crate) mod utils;

fn main() {
    Application::new(1080, 720).run();
    // Application::new(1920, 1440).run();
    // Application::default().run();
}
