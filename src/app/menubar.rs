use fltk::app::Sender;
use fltk::enums::{Color, Font, FrameType, Shortcut};
use fltk::menu::{MenuFlag, MenuItem, SysMenuBar, WindowMenuStyle};
use fltk::prelude::{GroupExt, MenuExt, WidgetExt};
use fltk::window::Window;

use crate::utils::consts::MENU_BAR_COLOR;
use crate::utils::enums::{AppEvent, AppMode};

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
    pub(crate) fn new(
        evt_sender: Sender<AppEvent>,
        p_x: i32,
        p_y: i32,
        p_w: i32,
        p_h: i32,
    ) -> Self {
        let mut window = Window::default().with_pos(p_x, p_y).with_size(p_w, p_h);
        window.set_color(MENU_BAR_COLOR);

        let mut menu_bar = SysMenuBar::default().with_size(p_w, p_h);
        menu_bar.set_color(MENU_BAR_COLOR);
        menu_bar.set_frame(FrameType::BorderFrame);

        window.end();
        SysMenuBar::set_window_menu_style(WindowMenuStyle::TabbingModePreferred);
        let editor = AppMenuBar::editor(&mut menu_bar, evt_sender.clone());
        let training = AppMenuBar::training(&mut menu_bar, evt_sender.clone());
        let settings = AppMenuBar::settings(&mut menu_bar, evt_sender.clone());
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
        apply_style(&mut menu_bar.at(editor).unwrap());
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
        apply_style(&mut menu_bar.at(training).unwrap());
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
        apply_style(&mut menu_bar.at(settings).unwrap());
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
        apply_style(&mut menu_bar.at(help).unwrap());
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

fn apply_style(item: &mut MenuItem) {
    item.set_label_color(Color::White);
    item.set_label_font(Font::HelveticaBold);
}
