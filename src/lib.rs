//! Void Money is a business simulator. It's goal is to teach economics, and let people explore
//! starting, buying, selling and running virtual businesses.
//!
//! # Introduction
//!
//! Upon entering this virtual economy, you are given a bit of starting virtual cash for it.
//! Will you start a business? Will you buy a business? Will you save this cash? Will you invest
//! it? Will you work for someone else? The decisions are yours to make!
//!
//! # Documents
//!
//! Below is a list of documents pertinent to the project.
//!
//! - [(OP) - Overview of Project](crate::docs::overview)
//! - [(SRS) - Software Requirement Specification](crate::docs::srs)
//! - [(SDD) - Software Design Document](crate::docs::sdd)
//!
//! # Technology
//!
//! Void Money runs in a web browser, using WebGL. The game simulates a virtual economy. People have
//! needs, and people use resources to create products to solve those needs. The economies are
//! dynamic, and ever changing.
//!
//! **Internal Technologies**
//!
//! - Anava Engine
//!
//! **External Languages and Frameworks**
//!
//! - Rust
//! - HTML, CSS, and JavaScript
//! - WebAssembly
//! - WebGL
//!
//! **External Crates**
//!
//! Please see the [Cargo.toml] file.
//!
//! # Disclaimer
//!
//! Void Money is not providing financial, business, legal, accounting or any other
//! kind of advise. You should consult proper professional advise if needed.
//!
//! By playing Void Money, you agree to the [Terms and Conditions]() as well as [Privacy Policy]().
//!
//! By viewing, downloading or using the source code for Void Money, you agree to the
//! License as included [within this repository]() or [the license on www.alexanderfarrell.com]().
//!

#![allow(dead_code)]
#![allow(unused_variables)]

#[macro_use]
pub extern crate wasm_bindgen;

use wasm_bindgen::JsCast;
pub use wasm_bindgen::prelude::*;
pub use web_sys::*;
pub use web_sys::WebGlRenderingContext as GL;

use engine::webgl_engine::WebGLEngine;

use crate::engine::base::app::App;
use crate::game::VoidMoney;

pub mod docs;
pub mod engine;
pub mod game;

/// The starting function of Void Money. When WASM is loaded, this function is called.
#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    let engine = WebGLEngine::new();
    let game = VoidMoney::new();

    let mut app = App::new(game, engine);

    app.start();

    Ok(())
}