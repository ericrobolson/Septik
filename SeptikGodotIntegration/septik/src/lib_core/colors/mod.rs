#[derive(Copy, Clone, Debug)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

pub type PalatteIndexType = u8;

#[derive(Clone)]
pub struct Palatte {
    colors: [Color; Self::MAX_COLORS],
}

impl Palatte {
    pub const MAX_COLORS: usize = 256;

    pub fn new() -> Self {
        return Self {
            colors: [Color { r: 0, g: 0, b: 0 }; Self::MAX_COLORS],
        };
    }
}
