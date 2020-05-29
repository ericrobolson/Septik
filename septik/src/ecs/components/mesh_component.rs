#[derive(Clone, Copy, PartialEq)]
pub enum Mesh {
    Monkey,
}

#[derive(Clone)]
pub struct MeshComponent {
    pub mesh: Mesh,
}

impl MeshComponent {
    pub fn new(mesh: Mesh) -> Self {
        Self { mesh: mesh }
    }
}
