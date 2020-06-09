use crate::ecs::World;
use crate::lib_core::{EngineInputs, InputType};

mod kiss3d;
mod three;

pub trait WindowGfx {
    /// Retrive all input since last poll from the Window
    fn poll_input(&mut self) -> Vec<InputType>;

    /// Render the given world on screen
    fn render(&mut self, world: &World);
}

pub struct WindowGfxBuilder {}
impl WindowGfxBuilder {
    /// Build the window/gfx
    pub fn build() -> Box<dyn WindowGfx> {
        return Box::new(three::ThreeRsBackend::new());
        //   return Box::new(kiss3d::Kiss3dBackend::new());
    }
}
