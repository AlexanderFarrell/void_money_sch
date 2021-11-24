use crate::engine::app::CoreComponent;
use crate::engine::visual::{ClearInfo, Visual};
use crate::engine::world::World;
use crate::{JsValue, JsCast, WebGlRenderingContext, WebGlRenderingContext as GL};
use crate::engine::builtin_engines::webgl_engine;

pub struct WebGLVisual {
    pub gl: WebGlRenderingContext,
    pub clear_info: ClearInfo,
}

impl WebGLVisual {
    pub fn new() -> Result<Self, JsValue> {
        let canvas_element = webgl_engine::element_by_id("game_canvas").unwrap();
        let canvas: web_sys::HtmlCanvasElement =
            canvas_element.dyn_into::<web_sys::HtmlCanvasElement>()?;

        let gl: WebGlRenderingContext = canvas.get_context("webgl")?.unwrap().dyn_into()?;

        let mut clear_info: ClearInfo = ClearInfo::default();
        clear_info.color.red = 0.5;
        clear_info.color.green = 0.2;
        clear_info.color.blue = 0.4;

        Ok(WebGLVisual {
            gl,
            clear_info
        })
    }
}

impl CoreComponent for WebGLVisual {
    fn setup(&mut self) {

    }

    fn breakdown(&mut self) {
        todo!()
    }
}

impl Visual for WebGLVisual {
    fn draw(&self, world: &World) {
        self.clear();
    }

    fn clear(&self) {
        self.gl.clear_color(
            self.clear_info.color.red,
            self.clear_info.color.green,
            self.clear_info.color.blue,
            self.clear_info.color.alpha
        );
        self.gl.clear_depth(self.clear_info.depth);
        self.gl.clear(GL::COLOR_BUFFER_BIT | GL::DEPTH_BUFFER_BIT);
    }

    fn get_clear_info(&self) -> &ClearInfo {
        todo!()
    }

    fn get_mut_clear_info(&self) -> &mut ClearInfo {
        todo!()
    }
}
