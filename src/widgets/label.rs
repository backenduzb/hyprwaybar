pub struct Label {
    pub text: String,
    pub background_color: Option<(f64, f64, f64, f64)>, // RGBA
}

impl Label {
    pub fn new(text: impl Into<String>) -> Self {
        Self { 
            text: text.into(),
            background_color: None,
        }
    }

    pub fn set_background(&mut self, r: f64, g: f64, b: f64, a: f64) {
        self.background_color = Some((r, g, b, a));
    }

    pub fn draw(&self) {
        println!("Label: {} | Background: {:?}", self.text, self.background_color);
        // Keyinchalik bu yerda cairo orqali chizish logikasi bo'ladi
    }
}