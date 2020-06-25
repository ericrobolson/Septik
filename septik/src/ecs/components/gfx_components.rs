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

#[derive(Clone)]
pub struct SpriteComponent {
    pub file_path: String,
}

impl SpriteComponent {
    pub fn new(file_path: String) -> Self {
        Self {
            file_path: file_path,
        }
    }
}
