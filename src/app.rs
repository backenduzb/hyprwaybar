use crate::hypr::connector;
use crate::hypr::sensors::time;
use crate::utils::{img_drawer, label_drawer};
use crate::hypr::sensors::application::{self, get_active_window};

pub fn run() {
    connector::run_connector(|cr, _state| {
        img_drawer::draw_image(&cr, "arch.png", 21, 20, 20.0, 6.0);
        if let Some(title) = get_active_window() {
            label_drawer::add_label(&cr, &title, 46.0, 7.5, (1.0, 1.0, 1.0), true);
        }        
        label_drawer::add_label(
            &cr,
            &time::get_time_string(),
            1835.0,
            7.5,
            (1.0, 1.0, 1.0),
            false,
        );
    });
}
