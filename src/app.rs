use crate::hypr::connector;
use crate::utils::{img_drawer, label_drawer};
use crate::hypr::sensors::time;

pub fn run(){
    connector::run_connector(|cr, _state|{
        img_drawer::draw_image(&cr, "arch.png", 21, 20, 20.0, 6.0);
        label_drawer::add_label(&cr, "Arch", 46.0, 22.0, (1.0,1.0,1.0));
        label_drawer::add_label(&cr, &time::get_time_string(), 1080.0, 22.0, (1.0,1.0,1.0));
    });
}