mod sdl2_backend_gl;
use crate::data_structures;
pub mod sys_events;
pub use sys_events::SysEvent;
mod sdl2_gl_pipeline;

type TextureId = u32;

pub type Position = data_structures::Aabb2d<u32>;

pub trait Backend {
    fn render(&mut self, node: Node);
    fn poll_events(&mut self) -> SysEvent;
}

pub struct Element {}

pub enum Node {
    Element(Element),
    Text(String),
    Empty,
}

pub fn build_backend() -> Box<dyn Backend> {
    let backend = sdl2_backend_gl::Sdl2BackendGL::new();
    return Box::new(backend);
}
