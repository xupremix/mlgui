use fltk::app::{App, Receiver, Scheme, Sender};
use fltk::prelude::{GroupExt, WidgetBase, WidgetExt, WindowExt};
use fltk::window::Window;

use crate::app::menubar::AppMenuBar;
use crate::settings::{AppEvent, AppMode, BG_COLOR, WINDOW_HEIGHT, WINDOW_TITLE, WINDOW_WIDTH};

mod mainview;
mod menubar;
mod rightview;
mod treeview;

pub(crate) struct Application {
    pub(crate) mode: AppMode,
    pub(crate) app: App,
    pub(crate) window: Window,
    pub(crate) evt_recv: Receiver<AppEvent>,
    pub(crate) evt_sender: Sender<AppEvent>,
    pub(crate) menu_bar: AppMenuBar,
}

impl Application {
    pub(crate) fn new() -> Application {
        let app = App::default().with_scheme(Scheme::Gtk);
        let mut window =
            Window::new(0, 0, WINDOW_WIDTH, WINDOW_HEIGHT, WINDOW_TITLE).center_screen();
        window.set_color(BG_COLOR);
        let (evt_sender, evt_recv) = fltk::app::channel();
        let mode = AppMode::Editor;
        let menu_bar = AppMenuBar::new(&mut window, evt_sender, mode);
        window.end();
        window.show();
        Application {
            mode,
            app,
            window,
            evt_recv,
            evt_sender,
            menu_bar,
        }
    }
    pub(crate) fn run(mut self) {
        while self.app.wait() {
            if let Some(evt) = self.evt_recv.recv() {
                match evt {
                    AppEvent::Editor => {
                        if self.mode != AppMode::Editor {
                            self.mode = AppMode::Editor;
                            self.menu_bar.redraw(self.mode);
                        }
                    }
                    AppEvent::Training => {
                        if self.mode != AppMode::Training {
                            self.mode = AppMode::Training;
                            self.menu_bar.redraw(self.mode);
                        }
                    }
                    AppEvent::Settings => {
                        eprintln!("Showing settings");
                    }
                    AppEvent::Help => {
                        eprintln!("Showing help");
                    }
                }
            }
        }
    }
}
