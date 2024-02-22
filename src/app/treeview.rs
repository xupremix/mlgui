use fltk::app::Sender;
use fltk::window::DoubleWindow;

use crate::settings::AppEvent;

pub(crate) fn init(window: &mut DoubleWindow, evt_sender: Sender<AppEvent>) {}
