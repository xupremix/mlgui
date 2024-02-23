use fltk::app::Sender;
use fltk::prelude::{GroupExt, WidgetBase, WidgetExt};
use fltk::window::Window;

use crate::app::mainview::editor::EditorView;
use crate::app::mainview::training::TrainingView;
use crate::settings::{AppEvent, AppMode, MENU_BAR_HEIGHT, WINDOW_HEIGHT, WINDOW_WIDTH};

mod editor;
mod training;

pub(crate) struct MainView {
    window: Window,
    evt_sender: Sender<AppEvent>,
    editor_view: EditorView,
    training_view: TrainingView,
}

fltk::widget_extends!(MainView, Window, window);

impl MainView {
    pub(crate) fn new(evt_sender: Sender<AppEvent>) -> Self {
        let window = Window::new(0, MENU_BAR_HEIGHT, WINDOW_WIDTH, WINDOW_HEIGHT, None);
        let training_view = TrainingView::new(evt_sender);
        let editor_view = EditorView::new(evt_sender);
        window.end();
        Self {
            window,
            evt_sender,
            editor_view,
            training_view,
        }
    }
    pub(crate) fn redraw_mode(&mut self, mode: AppMode) {
        match mode {
            AppMode::Editor => {
                self.editor_view.show();
                self.training_view.hide();
            }
            AppMode::Training => {
                self.editor_view.hide();
                self.training_view.show();
            }
        }
    }
}
