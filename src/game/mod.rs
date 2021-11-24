use crate::engine::app::{Game, Version};

pub struct VoidMoney {

}

impl VoidMoney {
    pub fn new() -> Self {
        VoidMoney {}
    }
}

impl Game for VoidMoney {
    fn setup(&mut self) {

    }

    fn update(&mut self) {

    }

    fn breakdown(&mut self) {

    }

    fn name(&self) -> String {
        todo!()
    }

    fn version(&self) -> Version {
        todo!()
    }
}
