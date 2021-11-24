use crate::engine::app::CoreComponent;
use crate::engine::world::World;
use web_sys::WebGlRenderingContext as GL;

#[derive(Copy, Clone)]
pub struct Color {
    pub red: f32,
    pub green: f32,
    pub blue: f32,
    pub alpha: f32,
}

impl Color {
    pub fn new(red: f32, green: f32, blue: f32, alpha: f32) -> Self {
        Color { red, green, blue, alpha }
    }

    pub fn new_rgb(red: f32, green: f32, blue: f32) -> Self {
        Color {red, green, blue, alpha: 1.0}
    }

    pub fn new_gray(brightness: f32) -> Self {
        Self {
            red: brightness,
            green: brightness,
            blue: brightness,
            alpha: 1.0
        }
    }
}

impl Default for Color {
    fn default() -> Self {
        Self {
            red: 0.0,
            green: 0.0,
            blue: 0.0,
            alpha: 1.0
        }
    }
}

pub struct ClearInfo {
    pub color: Color,
    pub depth: f32,
}

impl Default for ClearInfo {
    fn default() -> Self {
        ClearInfo {
            color: Default::default(),
            depth: 1.0
        }
    }
}

pub trait Visual: CoreComponent {
    fn draw(&self, world: &World);
    fn clear(&self);
    fn get_clear_info(&self) -> &ClearInfo;
    fn get_mut_clear_info(&self) -> &mut ClearInfo;
}
