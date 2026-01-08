use cairo::Context;
use image::{imageops::FilterType, GenericImageView};

pub fn draw_image(cr: &Context, image_name: &str, width: u32, height: u32, x: f64, y: f64) {
    let img_path = format!(
        "{}/src/assets/img/{}",
        env!("CARGO_MANIFEST_DIR"),
        image_name
    );
    let img = image::open(img_path).unwrap().to_rgba8();

    let resized = image::imageops::resize(&img, width, height, FilterType::Lanczos3);
    let (w, h) = resized.dimensions();

    let mut data = Vec::with_capacity((w * h * 4) as usize);
    for pixel in resized.pixels() {
        let [r, g, b, a] = pixel.0;
        data.extend_from_slice(&[a, r, g, b]); 
    }

    let img_surface = cairo::ImageSurface::create_for_data(
        data,
        cairo::Format::ARgb32,
        w as i32,
        h as i32,
        (4 * w) as i32,
    )
    .unwrap();

    cr.set_source_surface(&img_surface, x, y);
    cr.paint();
}
