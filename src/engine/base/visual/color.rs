use crate::engine::base::visual::Color;

impl Color {
    pub fn new(red: f32, green: f32, blue: f32, alpha: f32) -> Self {
        Color { red, green, blue, alpha }
    }

    pub fn new_rgb(red: f32, green: f32, blue: f32) -> Self {
        Color { red, green, blue, alpha: 1.0 }
    }

    pub fn new_gray(brightness: f32) -> Self {
        Self {
            red: brightness,
            green: brightness,
            blue: brightness,
            alpha: 1.0,
        }
    }
}
