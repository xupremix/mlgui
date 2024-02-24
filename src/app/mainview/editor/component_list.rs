use std::time::{Duration, Instant};

use fltk::app::Sender;
use fltk::enums::{Color, Cursor, Event, Font, FrameType};
use fltk::frame::Frame;
use fltk::group::Group;
use fltk::prelude::{GroupExt, WidgetBase, WidgetExt};
use fltk::tree::{Tree, TreeItem};
use fltk::window::Window;

use crate::settings::{
    ACTIVATION_FUNCTIONS, AppEvent, BG_COLOR, LAYERS, MENU_BAR_COLOR, MENU_BAR_HEIGHT,
    WINDOW_HEIGHT,
};

pub(crate) struct ComponentList {
    pub(crate) window: Window,
    pub(crate) component_tree: Tree,
    pub(crate) tree_items: Vec<TreeItem>,
}

fltk::widget_extends!(ComponentList, Window, window);

impl ComponentList {
    pub(crate) fn new(evt_sender: Sender<AppEvent>) -> Self {
        let mut window = Window::new(0, 0, 205, WINDOW_HEIGHT - MENU_BAR_HEIGHT, None);
        window.set_color(Color::White);

        let mut group = Group::default().with_size(203, WINDOW_HEIGHT - MENU_BAR_HEIGHT);
        group.set_frame(FrameType::FlatBox);

        let mut custom_frame_border = Frame::new(0, 0, 203, 30, None);
        custom_frame_border.set_frame(FrameType::FlatBox);
        custom_frame_border.set_color(Color::White);

        let mut frame = Frame::new(0, 1, 203, 28, "Components");
        frame.set_frame(FrameType::FlatBox);
        frame.set_label_font(Font::HelveticaBold);
        frame.set_color(MENU_BAR_COLOR);
        frame.set_label_color(Color::White);

        let mut component_tree = Tree::default()
            .with_pos(0, 30)
            .with_size(203, WINDOW_HEIGHT - MENU_BAR_HEIGHT - 30);
        component_tree.set_color(BG_COLOR);
        component_tree.set_label_color(Color::White);
        component_tree.set_selection_color(Color::from_hex(0x3E4452));
        component_tree.set_margin_left(-5);
        component_tree.set_show_root(false);
        component_tree.set_line_spacing(10);

        let interval = Duration::from_millis(500);
        let mut last_click = Instant::now();
        let mut coords = (-1, -1);
        component_tree.handle(move |tree, event| match event {
            Event::Push => {
                let new_coords = fltk::app::event_coords();
                let instant = Instant::now();
                if coords == new_coords && instant - last_click < interval {
                    if let Some(item) = tree.find_clicked(true) {
                        let label = item.label().unwrap();
                        if LAYERS.contains(&label.as_str()) {
                            evt_sender.send(AppEvent::AddLayer(label));
                        } else if ACTIVATION_FUNCTIONS.contains(&label.as_str()) {
                            evt_sender.send(AppEvent::AddActivationFunction(label));
                        }
                    }
                } else {
                    coords = new_coords;
                    last_click = instant;
                }
                true
            }
            _ => false,
        });
        let layers = layers(&mut component_tree);
        let activation_functions = activation_functions(&mut component_tree);
        let tree_items = layers
            .iter()
            .chain(activation_functions.iter())
            .cloned()
            .collect();
        window.end();

        let mut enabled = false;
        let mut threshold = 200;
        window.handle(move |window, event| match event {
            Event::Push => {
                let coords = fltk::app::event_coords();
                enabled = coords.0 + window.x() > threshold;
                true
            }
            Event::Drag => {
                let coords = fltk::app::event_coords();
                if enabled {
                    threshold = coords.0 - 4;
                    window.resize(0, 0, coords.0 + window.x(), WINDOW_HEIGHT - MENU_BAR_HEIGHT);
                    group.resize(0, 0, coords.0 - 2, WINDOW_HEIGHT - MENU_BAR_HEIGHT);
                    let diff = coords.0 - window.x();
                }
                true
            }
            Event::Move => {
                let coords = fltk::app::event_coords();
                if coords.0 + window.x() > threshold {
                    fltk::draw::set_cursor(Cursor::E);
                } else {
                    fltk::draw::set_cursor(Cursor::Default);
                }
                true
            }
            Event::Leave => {
                fltk::draw::set_cursor(Cursor::Default);
                true
            }
            _ => false,
        });
        Self {
            window,
            component_tree,
            tree_items,
        }
    }
}

fn activation_functions(tree: &mut Tree) -> Vec<TreeItem> {
    let mut out = vec![{
        let mut first = tree.add("Activation Functions").unwrap();
        first.set_label_font(Font::HelveticaBold);
        first.set_label_bgcolor(Color::from_hex(0x21252B));
        first.set_label_color(Color::White);
        first.set_label_size(15);
        first
    }];
    ACTIVATION_FUNCTIONS.iter().for_each(|layer| {
        out.push(
            tree.add(&format!("Activation Functions/{}", layer))
                .unwrap(),
        )
    });
    for node in out.iter_mut().skip(1) {
        node.set_label_color(Color::White);
        node.set_label_font(Font::Helvetica);
        node.set_label_size(14);
    }
    out
}

fn layers(tree: &mut Tree) -> Vec<TreeItem> {
    let mut out = vec![{
        let mut first = tree.add("Layers").unwrap();
        first.set_label_font(Font::HelveticaBold);
        first.set_label_bgcolor(Color::from_hex(0x21252B));
        first.set_label_color(Color::White);
        first.set_label_size(15);
        first
    }];
    LAYERS
        .iter()
        .for_each(|layer| out.push(tree.add(&format!("Layers/{}", layer)).unwrap()));
    for node in out.iter_mut().skip(1) {
        node.set_label_color(Color::White);
        node.set_label_font(Font::Helvetica);
        node.set_label_size(14);
    }
    out
}
