use js_sys::Number;
use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen::closure::Closure;
use web_sys::{Element, WebGlRenderingContext};

use visual::WebGLVisual;

use crate::engine::base::app::{CoreComponent, Version};
use crate::engine::base::audio::Audio;
use crate::engine::base::engine::Engine;
use crate::engine::base::input::Input;
use crate::engine::base::visual::Visual;
use crate::engine::base::world::World;
use crate::GL;
use crate::wasm_bindgen;

mod visual;
mod ui;
mod time;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    pub fn log(s: &str);

    #[wasm_bindgen(js_namespace = Date)]
    fn now() -> Number;
}

pub fn date_now() -> u64 {
    return now().as_f64().unwrap().round() as u64;
}

pub struct WebGLEngine {
    is_running: bool,
    visual: WebGLVisual,
    world: World,
}

impl WebGLEngine {
    pub fn new() -> Self {
        console_error_panic_hook::set_once();
        log("Initialized");
        log(format!("{}", date_now()).as_str());
        Self {
            is_running: false,
            visual: WebGLVisual::new().unwrap(),
            world: World::new(),
        }
    }
}

impl Engine for WebGLEngine {
    fn setup(&mut self) {
        log("Set up");
        self.is_running = true;
        self.visual.setup();
    }

    fn update(&mut self) {}

    fn draw(&mut self) {
        self.visual.draw(&self.world);
    }

    fn breakdown(&mut self) {}

    fn name(&self) -> String {
        String::from("WebGL Engine - MorphSight")
    }

    fn version(&self) -> Version {
        todo!()
    }

    fn platform(&self) -> String {
        String::from("WebGL")
    }

    fn running(&self) -> bool {
        self.is_running
    }

    fn stop(&mut self) {
        self.is_running = false;
    }

    fn game_iter(&self) -> fn(&Closure<dyn FnMut()>) {
        log("Requested animation frame");
        request_animation_frame
    }

    fn visual(&mut self) -> &dyn Visual {
        todo!()
    }

    fn audio(&mut self) -> &dyn Audio {
        todo!()
    }

    fn input(&mut self) -> &dyn Input {
        todo!()
    }

    fn world(&mut self) -> &mut World {
        todo!()
    }
}

#[inline]
pub fn window() -> web_sys::Window {
    web_sys::window().expect("Could not get window")
}

#[inline]
pub fn document() -> web_sys::Document {
    window().document().expect("Could not get document")
}

#[inline]
pub fn body() -> web_sys::HtmlElement { document().body().expect("Could not get body") }

#[inline]
pub fn element_by_id(id: &'static str) -> Option<Element> {
    document().get_element_by_id(id)
}

#[inline]
pub fn request_animation_frame(f: &Closure<dyn FnMut()>) {
    window()
        .request_animation_frame(f.as_ref().unchecked_ref())
        .expect("Could not request animation frame. ");
}

