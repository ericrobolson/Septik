use crate::ecs::World;
use crate::lib_core::{EngineInputs, InputType};

pub mod three;

pub trait WindowGfx {
    fn poll_input(&mut self) -> Vec<InputType>;
    fn render(&mut self, world: &World);
}

pub struct WindowGfxBuilder {}
impl WindowGfxBuilder {
    pub fn build() -> Box<dyn WindowGfx> {
        return Box::new(three::ThreeRsBackend::new());
    }
}
