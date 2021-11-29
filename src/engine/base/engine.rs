use wasm_bindgen::closure::Closure;

use crate::engine::base::app::Version;
use crate::engine::base::audio::Audio;
use crate::engine::base::input::Input;
use crate::engine::base::visual::Visual;
use crate::engine::base::world::World;

pub trait Engine {
    fn setup(&mut self);
    fn update(&mut self);
    fn draw(&mut self);
    fn breakdown(&mut self);
    fn name(&self) -> String;
    fn version(&self) -> Version;
    fn platform(&self) -> String;
    fn running(&self) -> bool;
    fn stop(&mut self);
    fn game_iter(&self) -> fn(&Closure<dyn FnMut()>);
    fn visual(&mut self) -> &dyn Visual;
    fn audio(&mut self) -> &dyn Audio;
    fn input(&mut self) -> &dyn Input;
    fn world(&mut self) -> &mut World;
}

// pub struct Output;
