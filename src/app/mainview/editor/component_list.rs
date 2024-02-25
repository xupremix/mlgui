use std::time::{Duration, Instant};

use fltk::app::Sender;
use fltk::enums::{Color, Cursor, Event, Font, FrameType};
use fltk::frame::Frame;
use fltk::group::Group;
use fltk::prelude::{GroupExt, WidgetBase, WidgetExt};
use fltk::tree::{Tree, TreeItem};
use fltk::window::Window;

use crate::utils::{
    ACTIVATION_FUNCTIONS, AppEvent, BG_COLOR, DRAG_THRESHOLD, LAYERS, MENU_BAR_COLOR,
    MENU_BAR_RATIO,
};

pub(crate) struct ComponentList {
    pub(crate) window: Window,
    pub(crate) component_tree: Tree,
    pub(crate) tree_items: Vec<TreeItem>,
}

fltk::widget_extends!(ComponentList, Window, window);

impl ComponentList {
    pub(crate) fn new(evt_sender: Sender<AppEvent>, p_w: i32, p_h: i32) -> Self {
        let mut window = Window::default().with_size(p_w, p_h);
        window.set_color(Color::White);

        let mut custom_frame_border = Frame::default().with_size(p_w, p_h / MENU_BAR_RATIO);
        custom_frame_border.set_frame(FrameType::FlatBox);
        custom_frame_border.set_color(Color::White);

        let mut frame = Frame::default()
            .with_pos(0, 1)
            .with_size(p_w - 2, p_h / MENU_BAR_RATIO - 2)
            .with_label("Components");
        frame.set_frame(FrameType::FlatBox);
        frame.set_label_font(Font::HelveticaBold);
        frame.set_color(MENU_BAR_COLOR);
        frame.set_label_color(Color::White);

        let mut group = Group::default()
            .with_pos(0, p_h / MENU_BAR_RATIO)
            .with_size(p_w - 2, p_h);
        let mut component_tree = Tree::default()
            .with_pos(0, p_h / MENU_BAR_RATIO)
            .with_size(p_w - 2, p_h);
        component_tree.set_color(BG_COLOR);
        component_tree.set_label_color(Color::White);
        component_tree.set_selection_color(Color::from_hex(0x3E4452));
        component_tree.set_margin_left(-5);
        component_tree.set_show_root(false);
        component_tree.set_line_spacing(10);
        group.end();

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
        window.handle(move |window, event| match event {
            Event::Push => {
                let x = fltk::app::event_x();
                enabled = window.w() - x < DRAG_THRESHOLD;
                true
            }
            Event::Drag if enabled => {
                let x = fltk::app::event_x();
                window.resize(0, 0, x, p_h);
                custom_frame_border.resize(0, 0, x, p_h / MENU_BAR_RATIO);
                frame.resize(0, 1, x - 2, p_h / MENU_BAR_RATIO - 2);
                group.resize(0, p_h / MENU_BAR_RATIO, x - 2, p_h);
                true
            }
            Event::Move => {
                let x = fltk::app::event_x();
                if window.w() - x < DRAG_THRESHOLD {
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
        first.set_label_bgcolor(MENU_BAR_COLOR);
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
        first.set_label_bgcolor(MENU_BAR_COLOR);
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
