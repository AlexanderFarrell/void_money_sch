use web_sys::WebGlRenderingContext as GL;

use crate::engine::base::app::CoreComponent;
use crate::engine::base::world::World;

pub mod color;
pub mod visible_object;
pub mod shader;
pub mod mesh;
pub mod material;
pub mod context;

/// The visual output to the user (Graphics)
pub trait Visual: CoreComponent {
    /// Draws a frame of the game world.
    fn draw(&self, world: &World);

    /// Clears a frame, resetting color and depth information for the next frame. Performed at the
    /// beginning of each draw.
    fn clear(&self);

    /// Gets information on how the frames are cleared, or in other words reset.
    fn get_clear_info(&self) -> &ClearInfo;

    /// Gets information **which can be modified** on how the frames are cleared,
    /// or in other words reset.
    fn get_mut_clear_info(&self) -> &mut ClearInfo;
}

pub struct VisibleObject {

}

pub trait AttributeBuffer {

}

pub trait UniformBuffer {

}

pub trait Material {

}

pub trait Mesh {

}

pub struct ClayMesh {

}

#[derive(Copy, Clone)]
pub struct Color {
    pub red: f32,
    pub green: f32,
    pub blue: f32,
    pub alpha: f32,
}

impl Default for Color {
    fn default() -> Self {
        Self {
            red: 0.0,
            green: 0.0,
            blue: 0.0,
            alpha: 1.0,
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
            depth: 1.0,
        }
    }
}