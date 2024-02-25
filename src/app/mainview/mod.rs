use fltk::app::Sender;
use fltk::prelude::{GroupExt, WidgetBase, WidgetExt};
use fltk::window::Window;

use crate::app::mainview::editor::EditorView;
use crate::app::mainview::training::TrainingView;
use crate::utils::{
    AppEvent, AppMode,
};

mod editor;
mod training;

pub(crate) struct MainView {
    window: Window,
    editor_view: EditorView,
    training_view: TrainingView,
}

fltk::widget_extends!(MainView, Window, window);

impl MainView {
    pub(crate) fn new(
        evt_sender: Sender<AppEvent>,
        p_x: i32,
        p_y: i32,
        p_w: i32,
        p_h: i32,
    ) -> Self {
        let window = Window::default().with_pos(p_x, p_y).with_size(p_w, p_h);

        let training_view = TrainingView::new(evt_sender.clone(), p_w, p_h);
        let editor_view = EditorView::new(evt_sender, p_w, p_h);
        window.end();
        Self {
            window,
            editor_view,
            training_view,
        }
    }
    pub(crate) fn add_layer(&self, layer: String) {
        eprintln!("Adding layer: {}", layer);
    }
    pub(crate) fn add_activation_fn(&self, activation_fn: String) {
        eprintln!("Adding activation fn: {}", activation_fn);
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
