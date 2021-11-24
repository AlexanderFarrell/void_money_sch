use wasm_bindgen::closure::Closure;
use crate::engine::app::{CoreComponent, Version};
use crate::engine::audio::Audio;
use crate::engine::input::Input;
use crate::engine::visual::Visual;
use crate::engine::world::World;
use crate::wasm_bindgen;

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
