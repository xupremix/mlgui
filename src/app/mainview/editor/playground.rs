use std::cell::RefCell;
use std::path::PathBuf;
use std::rc::Rc;

use fltk::app::MouseButton;
use fltk::enums::Event;
use fltk::enums::FrameType;
use fltk::enums::{Align, Color};
use fltk::frame::Frame;
use fltk::group::Group;
use fltk::image::{PngImage, SvgImage};
use fltk::prelude::{GroupExt, ImageExt, WidgetBase, WidgetExt};
use fltk::window::Window;
use tch::Device;

use crate::components::NNComponent;
use crate::utils::consts::{BASE_COMPONENT_HEIGHT, BASE_COMPONENT_WIDTH, BG_COLOR};
use crate::utils::loss_fn::LossFunction;
use crate::utils::CustomDialog;

pub(crate) struct Playground {
    draw_area: Rc<RefCell<Window>>,
    components: Rc<RefCell<Vec<NNComponent>>>,
    first: Rc<RefCell<Option<usize>>>,
    first_component: Rc<RefCell<Option<Frame>>>,
}

impl Playground {
    pub(crate) fn new(p_w: i32, p_h: i32) -> Self {
        let mut draw_area = Window::default().with_size(p_w, p_h);
        draw_area.set_color(BG_COLOR);
        draw_area.end();

        Self {
            draw_area: Rc::new(RefCell::new(draw_area)),
            components: Rc::new(RefCell::new(vec![])),
            first: Rc::new(RefCell::new(None)),
            first_component: Rc::new(RefCell::new(None)),
        }
    }

    pub(crate) fn add_component(&mut self, nn_comp: NNComponent) {
        let i = self.components.borrow().len();
        let draw_area = self.draw_area.clone();
        let set_first_ref = self.first.clone();
        let first_component = self.first_component.clone();
        let elems = self.components.clone();
        let label = *nn_comp;
        let mut g = Group::default()
            .with_size(
                BASE_COMPONENT_WIDTH.max(label.len() as i32 * 12),
                BASE_COMPONENT_HEIGHT,
            )
            .center_of(&*draw_area.borrow());
        let mut component = Frame::default()
            .with_size(g.w(), g.h())
            .center_of(&*draw_area.borrow());
        component.set_frame(FrameType::EngravedBox);
        component.set_color(Color::White);
        let mut inner_component = Frame::default()
            .with_size(component.w() - 4, component.h() - 4)
            .center_of(&component)
            .with_label(*nn_comp);
        inner_component.set_label_color(Color::White);
        inner_component.set_frame(FrameType::FlatBox);
        inner_component.set_color(BG_COLOR);
        // configured sign
        let mut img = SvgImage::load("src/assets/cross.svg").unwrap();
        inner_component.draw(move |f| {
            img.scale(15, 15, true, true);
            img.draw(
                f.x() + f.w() - img.w(),
                f.y() + f.h() - img.h(),
                img.w(),
                img.h(),
            );
        });
        g.end();
        draw_area.borrow_mut().add(&g);
        draw_area.borrow_mut().redraw();
        let mut set = false;
        let mut prev = (-1, -1);
        let inner_component_handler = Rc::new(RefCell::new(Some(inner_component)));
        let mv_inner_component = inner_component_handler.clone();
        inner_component_handler
            .borrow_mut()
            .as_mut()
            .unwrap()
            .handle(move |_, event| match event {
                Event::Push => {
                    match fltk::app::event_mouse_button() {
                        MouseButton::Left => {
                            set = true;
                            prev = fltk::app::event_coords();
                        }
                        MouseButton::Right => {
                            let coords = fltk::app::event_coords();
                            let mut group = Group::default()
                                .with_pos(coords.0, coords.1)
                                .with_size(75, 61);
                            let mut configuration_window = Frame::default()
                                .with_size(group.w(), group.h())
                                .center_of(&group);
                            configuration_window.set_color(Color::White);
                            configuration_window.set_frame(FrameType::FlatBox);
                            let mut set_first_frame = Frame::default()
                                .with_pos(
                                    configuration_window.x() + 1,
                                    configuration_window.y() + 1,
                                )
                                .with_size(
                                    configuration_window.w() - 2,
                                    configuration_window.h() / 2 - 1,
                                )
                                .with_label("Set First");
                            set_first_frame.set_frame(FrameType::FlatBox);
                            set_first_frame.set_label_color(Color::White);
                            set_first_frame.set_align(Align::Center);
                            set_first_frame.set_color(BG_COLOR);
                            let mut config_frame = Frame::default()
                                .with_pos(
                                    set_first_frame.x(),
                                    set_first_frame.y() + set_first_frame.h() + 1,
                                )
                                .with_size(set_first_frame.w(), set_first_frame.h())
                                .with_label("Configure");
                            config_frame.set_frame(FrameType::FlatBox);
                            config_frame.set_label_color(Color::White);
                            config_frame.set_align(Align::Center);
                            config_frame.set_color(BG_COLOR);
                            group.end();
                            let mv_inner_component = mv_inner_component.clone();
                            let mv_inner_config = mv_inner_component.clone();
                            let mv_first_component = first_component.clone();
                            let mv_elems = elems.clone();
                            let config_elems = mv_elems.clone();
                            let c = draw_area.clone();
                            let set_first_ref = set_first_ref.clone();
                            let set_first_ref_config = set_first_ref.clone();
                            set_first_frame.handle(move |_, evt| match evt {
                                Event::Push => {
                                    let inner_elem = mv_elems.clone();
                                    let prev_i = *set_first_ref.borrow();
                                    if prev_i.is_none() || prev_i.is_some() && prev_i.unwrap() != i
                                    {
                                        let configured = if let Some(prev_i) =
                                            set_first_ref.borrow_mut().replace(i)
                                        {
                                            let e = mv_elems.borrow().get(prev_i).cloned().unwrap();
                                            match e {
                                                NNComponent::Layer { configured, .. } => configured,
                                                NNComponent::ActivationFunction { .. } => true,
                                            }
                                        } else {
                                            false
                                        };
                                        mv_inner_component.borrow_mut().as_mut().unwrap().draw(
                                            move |f| {
                                                let mut first =
                                                    PngImage::load("src/assets/star.png").unwrap();
                                                first.scale(15, 15, true, true);
                                                first.draw(f.x(), f.y(), first.w(), first.h());
                                                if configured {
                                                    let mut first =
                                                        SvgImage::load("src/assets/tick.svg")
                                                            .unwrap();
                                                    first.scale(15, 15, true, true);
                                                    first.draw(
                                                        f.x() + f.w() - first.w(),
                                                        f.y() + f.h() - first.h(),
                                                        first.w(),
                                                        first.h(),
                                                    );
                                                } else {
                                                    let mut first =
                                                        SvgImage::load("src/assets/cross.svg")
                                                            .unwrap();
                                                    first.scale(15, 15, true, true);
                                                    first.draw(
                                                        f.x() + f.w() - first.w(),
                                                        f.y() + f.h() - first.h(),
                                                        first.w(),
                                                        first.h(),
                                                    );
                                                }
                                            },
                                        );
                                        if let Some(f) = mv_first_component.borrow_mut().as_mut() {
                                            f.draw(move |f| {
                                                let e =
                                                    inner_elem.borrow().get(i).cloned().unwrap();
                                                let configured = match e {
                                                    NNComponent::Layer { configured, .. } => {
                                                        configured
                                                    }
                                                    NNComponent::ActivationFunction { .. } => true,
                                                };
                                                if configured {
                                                    let mut first =
                                                        SvgImage::load("src/assets/tick.svg")
                                                            .unwrap();
                                                    first.scale(15, 15, true, true);
                                                    first.draw(
                                                        f.x() + f.w() - first.w(),
                                                        f.y() + f.h() - first.h(),
                                                        first.w(),
                                                        first.h(),
                                                    );
                                                } else {
                                                    let mut first =
                                                        SvgImage::load("src/assets/cross.svg")
                                                            .unwrap();
                                                    first.scale(15, 15, true, true);
                                                    first.draw(
                                                        f.x() + f.w() - first.w(),
                                                        f.y() + f.h() - first.h(),
                                                        first.w(),
                                                        first.h(),
                                                    );
                                                }
                                            });
                                        }
                                        mv_first_component
                                            .replace(mv_inner_component.borrow().clone());
                                        c.borrow_mut().redraw();
                                    }
                                    true
                                }
                                _ => false,
                            });
                            config_frame.handle(move |_, evt| match evt {
                                Event::Push => {
                                    if let NNComponent::Layer { configured, .. } =
                                        config_elems.borrow_mut().get_mut(i).unwrap()
                                    {
                                        *configured = true;
                                    }
                                    let first_ref = set_first_ref_config.clone();
                                    let config_elems = config_elems.clone();
                                    mv_inner_config
                                        .borrow_mut()
                                        .as_mut()
                                        .unwrap()
                                        .draw(move |f| {
                                            let first_ref = first_ref.borrow();
                                            if first_ref.is_some() && first_ref.unwrap() == i {
                                                let mut first =
                                                    PngImage::load("src/assets/star.png").unwrap();
                                                first.scale(15, 15, true, true);
                                                first.draw(f.x(), f.y(), first.w(), first.h());
                                            }
                                            let e = config_elems.borrow().get(i).cloned().unwrap();
                                            let configured = match e {
                                                NNComponent::Layer { configured, .. } => configured,
                                                NNComponent::ActivationFunction { .. } => true,
                                            };
                                            if configured {
                                                let mut first =
                                                    SvgImage::load("src/assets/tick.svg").unwrap();
                                                first.scale(15, 15, true, true);
                                                first.draw(
                                                    f.x() + f.w() - first.w(),
                                                    f.y() + f.h() - first.h(),
                                                    first.w(),
                                                    first.h(),
                                                );
                                            } else {
                                                let mut first =
                                                    SvgImage::load("src/assets/cross.svg").unwrap();
                                                first.scale(15, 15, true, true);
                                                first.draw(
                                                    f.x() + f.w() - first.w(),
                                                    f.y() + f.h() - first.h(),
                                                    first.w(),
                                                    first.h(),
                                                );
                                            }
                                        });
                                    true
                                }
                                _ => false,
                            });
                            let c = draw_area.clone();
                            group.handle(move |group, event| match event {
                                Event::Leave => {
                                    c.borrow_mut().remove(group);
                                    c.borrow_mut().redraw();
                                    true
                                }
                                _ => false,
                            });
                            draw_area.borrow_mut().add(&group);
                            draw_area.borrow_mut().redraw();
                        }
                        _ => {}
                    }
                    true
                }
                Event::Drag if set => {
                    let new_coords = fltk::app::event_coords();
                    let (new_x, new_y) = (
                        component.x() + (new_coords.0 - prev.0),
                        component.y() + (new_coords.1 - prev.1),
                    );
                    prev = new_coords;
                    g.set_pos(new_x, new_y);
                    draw_area.borrow_mut().redraw();
                    true
                }
                _ => false,
            });
        self.components.borrow_mut().push(nn_comp);
    }

    pub(crate) fn build_model(
        &self,
        save_path: PathBuf,
        device: Device,
        optimizer: String,
        loss_fn: LossFunction,
        lr: f64,
        batch_size: i64,
        epochs: usize,
    ) {
        CustomDialog::show(
            220,
            40,
            "Success",
            "Model built successfully",
            BG_COLOR,
            Color::Green,
        );
        eprintln!("Building model");
        eprintln!("Save path: {:?}", save_path);
        eprintln!("Device: {:?}", device);
        eprintln!("Optimizer: {}", optimizer);
        eprintln!("Loss function: {:?}", loss_fn);
        eprintln!("Learning rate: {}", lr);
        eprintln!("Batch size: {}", batch_size);
        eprintln!("Epochs: {}", epochs);
        eprintln!("Components: {:#?}", self.components);
    }
}
