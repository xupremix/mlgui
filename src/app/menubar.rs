use fltk::app::Sender;
use fltk::enums::{Color, FrameType, Shortcut};
use fltk::menu::{MenuFlag, SysMenuBar, WindowMenuStyle};
use fltk::prelude::{GroupExt, MenuExt, WidgetBase, WidgetExt};
use fltk::window::Window;

use crate::settings::{AppEvent, AppMode, MENU_BAR_COLOR, MENU_BAR_HEIGHT, WINDOW_WIDTH};

pub(crate) struct AppMenuBar {
    window: Window,
    menu_bar: SysMenuBar,
    editor: i32,
    training: i32,
    settings: i32,
    help: i32,
}

fltk::widget_extends!(AppMenuBar, Window, window);

impl AppMenuBar {
    pub(crate) fn new(evt_sender: Sender<AppEvent>) -> Self {
        let mut window = Window::new(0, 0, WINDOW_WIDTH, MENU_BAR_HEIGHT, None);
        window.set_color(MENU_BAR_COLOR);
        let mut menu_bar = SysMenuBar::default().with_size(WINDOW_WIDTH, MENU_BAR_HEIGHT);
        window.end();
        menu_bar.set_color(MENU_BAR_COLOR);
        menu_bar.set_frame(FrameType::BorderFrame);
        SysMenuBar::set_window_menu_style(WindowMenuStyle::TabbingModePreferred);
        let editor = AppMenuBar::editor(&mut menu_bar, evt_sender);
        let training = AppMenuBar::training(&mut menu_bar, evt_sender);
        let settings = AppMenuBar::settings(&mut menu_bar, evt_sender);
        let help = AppMenuBar::help(&mut menu_bar, evt_sender);
        Self {
            window,
            menu_bar,
            editor,
            training,
            settings,
            help,
        }
    }
    fn editor(menu_bar: &mut SysMenuBar, evt_sender: Sender<AppEvent>) -> i32 {
        let editor = menu_bar.add_emit(
            "Editor",
            Shortcut::Ctrl | Shortcut::Shift | 'e',
            MenuFlag::Inactive | MenuFlag::MenuDivider,
            evt_sender,
            AppEvent::Editor,
        );
        let mut editor_widget = menu_bar.at(editor).unwrap();
        editor_widget.set_label_color(Color::White);
        editor
    }
    fn training(menu_bar: &mut SysMenuBar, evt_sender: Sender<AppEvent>) -> i32 {
        let training = menu_bar.add_emit(
            "Training",
            Shortcut::Ctrl | Shortcut::Shift | 't',
            MenuFlag::Normal | MenuFlag::MenuDivider,
            evt_sender,
            AppEvent::Training,
        );
        let mut training_widget = menu_bar.at(training).unwrap();
        training_widget.set_label_color(Color::White);
        training
    }
    fn settings(menu_bar: &mut SysMenuBar, evt_sender: Sender<AppEvent>) -> i32 {
        let settings = menu_bar.add_emit(
            "Settings",
            Shortcut::Control | Shortcut::Shift | 's',
            MenuFlag::Normal | MenuFlag::MenuDivider,
            evt_sender,
            AppEvent::Settings,
        );
        let mut training_widget = menu_bar.at(settings).unwrap();
        training_widget.set_label_color(Color::White);
        settings
    }
    fn help(menu_bar: &mut SysMenuBar, evt_sender: Sender<AppEvent>) -> i32 {
        let help = menu_bar.add_emit(
            "Help",
            Shortcut::Control | Shortcut::Shift | 'h',
            MenuFlag::Normal | MenuFlag::MenuDivider,
            evt_sender,
            AppEvent::Help,
        );
        let mut training_widget = menu_bar.at(help).unwrap();
        training_widget.set_label_color(Color::White);
        help
    }
    pub(crate) fn redraw_mode(&mut self, mode: AppMode) {
        let mut editor = self.menu_bar.at(self.editor).unwrap();
        let mut training = self.menu_bar.at(self.training).unwrap();
        match mode {
            AppMode::Editor => {
                eprintln!("Editor mode");
                editor.deactivate();
                training.activate();
            }
            AppMode::Training => {
                eprintln!("Training mode");
                editor.activate();
                training.deactivate();
            }
        }
        self.redraw();
    }
}
