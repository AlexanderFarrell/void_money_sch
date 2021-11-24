#![allow(dead_code)]
#![allow(unused_variables)]

#[macro_use]
pub extern crate wasm_bindgen;
pub use wasm_bindgen::prelude::*;
pub use web_sys::*;
use wasm_bindgen::JsCast;
pub use web_sys::WebGlRenderingContext as GL;
use crate::engine::app::App;
use crate::engine::builtin_engines::webgl_engine::WebGLEngine;
use crate::game::VoidMoney;

pub mod docs;
pub mod engine;
pub mod game;

#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    let engine = WebGLEngine::new();
    let game = VoidMoney::new();

    let mut app = App::new(game, engine);

    app.start();

    Ok(())
}