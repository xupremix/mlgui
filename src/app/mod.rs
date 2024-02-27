use fltk::app::{App, Receiver, Scheme};
use fltk::prelude::{GroupExt, WidgetBase, WidgetExt, WindowExt};
use fltk::window::Window;

use crate::app::mainview::MainView;
use crate::app::menubar::AppMenuBar;
use crate::utils::consts::{MENU_BAR_RATIO, WINDOW_TITLE};
use crate::utils::enums::{AppEvent, AppMode};

mod mainview;
mod menubar;

pub(crate) struct Application {
    pub(crate) mode: AppMode,
    pub(crate) app: App,
    pub(crate) window: Window,
    pub(crate) evt_recv: Receiver<AppEvent>,
    pub(crate) menu_bar: AppMenuBar,
    pub(crate) main_view: MainView,
}

impl Application {
    pub(crate) fn new(width: i32, height: i32) -> Application {
        let app = App::default().with_scheme(Scheme::Gtk);
        let mut window = Window::new(0, 0, width, height, WINDOW_TITLE).center_screen();
        let (evt_sender, evt_recv) = fltk::app::channel();
        let menu_bar = AppMenuBar::new(evt_sender.clone(), 0, 0, width, height / MENU_BAR_RATIO);
        let main_view = MainView::new(evt_sender, 0, menu_bar.h(), width, height - menu_bar.h());
        window.end();
        window.set_callback(move |_| {
            app.quit();
        });
        Application {
            mode: AppMode::Editor,
            app,
            window,
            evt_recv,
            menu_bar,
            main_view,
        }
    }
    pub(crate) fn run(mut self) {
        self.window.show();
        while self.app.wait() {
            if let Some(evt) = self.evt_recv.recv() {
                match evt {
                    AppEvent::Editor => {
                        if self.mode != AppMode::Editor {
                            self.mode = AppMode::Editor;
                            self.main_view.redraw_mode(self.mode);
                            self.menu_bar.redraw_mode(self.mode);
                        }
                    }
                    AppEvent::Training => {
                        if self.mode != AppMode::Training {
                            self.mode = AppMode::Training;
                            self.main_view.redraw_mode(self.mode);
                            self.menu_bar.redraw_mode(self.mode);
                        }
                    }
                    AppEvent::Settings => {
                        eprintln!("Showing settings");
                    }
                    AppEvent::Help => {
                        eprintln!("Showing help");
                    }
                    AppEvent::AddLayer(layer) => {
                        self.main_view.add_layer(layer);
                    }
                    AppEvent::AddActivationFunction(activation_fn) => {
                        self.main_view.add_activation_fn(activation_fn);
                    }
                }
            }
        }
    }
}

impl Default for Application {
    fn default() -> Self {
        let (w, h) = fltk::app::screen_size();
        Self::new(w as i32, h as i32)
    }
}
