use crate::engine::base::app::{Game, Version};
use crate::engine::base::engine::Engine;

pub mod econ;

pub struct VoidMoney {}

impl VoidMoney {
    pub fn new() -> Self {
        VoidMoney {}
    }
}

impl Game for VoidMoney {
    fn setup(&mut self, engine: &mut dyn Engine) {}

    fn update(&mut self, engine: &mut dyn Engine) {}

    fn breakdown(&mut self, engine: &mut dyn Engine) {}

    fn name(&self) -> String {
        todo!()
    }

    fn version(&self) -> Version {
        todo!()
    }
}
