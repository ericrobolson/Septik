pub struct Color {}

pub type PalatteIndexType = u8;
pub struct Palatte {
    colors: [Color; Self::MAX_COLORS],
}

impl Palatte {
    pub const MAX_COLORS: usize = 256;
}
