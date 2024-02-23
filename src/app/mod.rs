use fltk::app::{App, Receiver, Scheme, Sender};
use fltk::prelude::{GroupExt, WidgetBase, WidgetExt, WindowExt};
use fltk::window::Window;

use crate::app::mainview::MainView;
use crate::app::menubar::AppMenuBar;
use crate::settings::{AppEvent, AppMode, WINDOW_HEIGHT, WINDOW_TITLE, WINDOW_WIDTH};

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
    pub(crate) main_view: MainView,
}

impl Application {
    pub(crate) fn new() -> Application {
        let app = App::default().with_scheme(Scheme::Gtk);
        let mut window =
            Window::new(0, 0, WINDOW_WIDTH, WINDOW_HEIGHT, WINDOW_TITLE).center_screen();
        let (evt_sender, evt_recv) = fltk::app::channel();
        let menu_bar = AppMenuBar::new(evt_sender);
        let main_view = MainView::new(evt_sender);
        window.end();
        window.show();
        Application {
            mode: AppMode::Editor,
            app,
            window,
            evt_recv,
            evt_sender,
            menu_bar,
            main_view,
        }
    }
    pub(crate) fn run(mut self) {
        while self.app.wait() {
            if let Some(evt) = self.evt_recv.recv() {
                match evt {
                    AppEvent::Editor => {
                        if self.mode != AppMode::Editor {
                            self.mode = AppMode::Editor;
                        } else {
                            continue;
                        }
                    }
                    AppEvent::Training => {
                        if self.mode != AppMode::Training {
                            self.mode = AppMode::Training;
                        } else {
                            continue;
                        }
                    }
                    AppEvent::Settings => {
                        eprintln!("Showing settings");
                        continue;
                    }
                    AppEvent::Help => {
                        eprintln!("Showing help");
                        continue;
                    }
                }
                self.menu_bar.redraw_mode(self.mode);
                self.main_view.redraw_mode(self.mode);
            }
        }
    }
}
