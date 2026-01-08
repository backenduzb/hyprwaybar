pub fn rounded_rect(cr: &cairo::Context, x: f64, y: f64, w: f64, h: f64, r: f64) {
    cr.new_sub_path();
    cr.arc(x + w - r, y + r, r, -90.0f64.to_radians(), 0.0);      
    cr.arc(x + w - r, y + h - r, r, 0.0, 90.0f64.to_radians());     
    cr.arc(x + r, y + h - r, r, 90.0f64.to_radians(), 180.0f64.to_radians());       
    cr.arc(x + r, y + r, r, 180.0f64.to_radians(), 270.0f64.to_radians());         
    cr.close_path();
}
