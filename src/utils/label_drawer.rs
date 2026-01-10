use crate::config::settings::TEXT_SIZE;

pub fn rounded_rect(cr: &cairo::Context, x: f64, y: f64, w: f64, h: f64, r: f64) {
    cr.new_sub_path();
    cr.arc(x + w - r, y + r, r, -90.0f64.to_radians(), 0.0);
    cr.arc(x + w - r, y + h - r, r, 0.0, 90.0f64.to_radians());
    cr.arc(
        x + r,
        y + h - r,
        r,
        90.0f64.to_radians(),
        180.0f64.to_radians(),
    );
    cr.arc(
        x + r,
        y + r,
        r,
        180.0f64.to_radians(),
        270.0f64.to_radians(),
    );
    cr.close_path();
}

pub fn add_label(cr: &cairo::Context, text: &str, x: f64, y: f64, color: (f64, f64, f64), this_bold: bool) {
    let (r, g, b) = color;
    cr.set_source_rgb(r, g, b);

    cr.select_font_face(
        "SF Pro Text",
        cairo::FontSlant::Normal,
        if this_bold { cairo::FontWeight::Bold } else { cairo::FontWeight::Normal },
    );
    cr.set_font_size(TEXT_SIZE);

    let extents = cr.font_extents().unwrap();
    let baseline = y + extents.ascent();
    
    cr.move_to(x, baseline);
    cr.show_text(text);
}
