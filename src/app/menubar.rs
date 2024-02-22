use fltk::app::Sender;
use fltk::enums::{Color, FrameType, Shortcut};
use fltk::menu::{MenuFlag, SysMenuBar, WindowMenuStyle};
use fltk::prelude::{GroupExt, MenuExt, WidgetBase, WidgetExt};
use fltk::window::DoubleWindow;

use crate::settings::{AppEvent, AppMode, MENU_BAR_COLOR, MENU_BAR_HEIGHT, WINDOW_WIDTH};

pub(crate) struct AppMenuBar {
    menu_bar: SysMenuBar,
    editor: i32,
    training: i32,
    settings: i32,
    help: i32,
}

impl AppMenuBar {
    pub(crate) fn new(
        window: &mut DoubleWindow,
        evt_sender: Sender<AppEvent>,
        mode: AppMode,
    ) -> Self {
        let mut menu_bar = SysMenuBar::new(0, 0, WINDOW_WIDTH, MENU_BAR_HEIGHT, None);
        SysMenuBar::set_window_menu_style(WindowMenuStyle::TabbingModePreferred);
        menu_bar.set_color(MENU_BAR_COLOR);
        menu_bar.set_frame(FrameType::BorderFrame);
        window.add(&menu_bar);
        let editor = AppMenuBar::editor(&mut menu_bar, evt_sender, mode);
        let training = AppMenuBar::training(&mut menu_bar, evt_sender, mode);
        let settings = AppMenuBar::settings(&mut menu_bar, evt_sender);
        let help = AppMenuBar::help(&mut menu_bar, evt_sender);
        Self {
            menu_bar,
            editor,
            training,
            settings,
            help,
        }
    }
    fn editor(menu_bar: &mut SysMenuBar, evt_sender: Sender<AppEvent>, mode: AppMode) -> i32 {
        let editor = menu_bar.add_emit(
            "Editor",
            Shortcut::Ctrl | Shortcut::Shift | 'e',
            if mode == AppMode::Training {
                MenuFlag::Normal
            } else {
                MenuFlag::Inactive
            } | MenuFlag::MenuDivider,
            evt_sender,
            AppEvent::Editor,
        );
        let mut editor_widget = menu_bar.at(editor).unwrap();
        editor_widget.set_label_color(Color::White);
        editor
    }
    fn training(menu_bar: &mut SysMenuBar, evt_sender: Sender<AppEvent>, mode: AppMode) -> i32 {
        let training = menu_bar.add_emit(
            "Training",
            Shortcut::Ctrl | Shortcut::Shift | 't',
            if mode == AppMode::Editor {
                MenuFlag::Normal
            } else {
                MenuFlag::Inactive
            } | MenuFlag::MenuDivider,
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
    pub(crate) fn redraw(&mut self, mode: AppMode) {
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
        self.menu_bar.redraw();
    }
}
