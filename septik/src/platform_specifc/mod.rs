use crate::ecs::World;
use crate::lib_core::{EngineInputs, InputType};

mod kiss3d;
mod three;

pub trait WindowGfx {
    fn poll_input(&mut self) -> Vec<InputType>;
    fn render(&mut self, world: &World);
}

pub struct WindowGfxBuilder {}
impl WindowGfxBuilder {
    pub fn build() -> Box<dyn WindowGfx> {
        //return Box::new(three::ThreeRsBackend::new());
        return Box::new(kiss3d::Kiss3dBackend::new());
    }
}
