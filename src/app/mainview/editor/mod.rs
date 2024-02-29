use std::cell::RefCell;
use std::rc::Rc;

use fltk::prelude::{GroupExt, WidgetBase, WidgetExt};
use fltk::window::{DoubleWindow, Window};

use crate::app::mainview::editor::component_list::ComponentList;
use crate::app::mainview::editor::configs::ConfingList;
use crate::app::mainview::editor::playground::Playground;
use crate::utils::consts::{COMPONENT_LIST_RATIO, CONFIG_LIST_RATIO};

mod component_list;
mod configs;
mod playground;

pub(crate) struct EditorView {
    window: DoubleWindow,
    graph: Rc<RefCell<Playground>>,
    comp_list: ComponentList,
    conf_list: ConfingList,
}

fltk::widget_extends!(EditorView, Window, window);

impl EditorView {
    pub(crate) fn new(p_w: i32, p_h: i32) -> Self {
        let window = Window::default().with_size(p_w, p_h);

        // playground
        let graph = Rc::new(RefCell::new(Playground::new(p_w, p_h)));

        // component list
        let comp_list = ComponentList::new(graph.clone(), p_w / COMPONENT_LIST_RATIO, p_h);

        // configs
        let conf_list = ConfingList::new(
            graph.clone(),
            p_w - p_w / CONFIG_LIST_RATIO,
            0,
            p_w / CONFIG_LIST_RATIO,
            p_h,
        );

        window.end();
        Self {
            window,
            graph,
            comp_list,
            conf_list,
        }
    }
}
