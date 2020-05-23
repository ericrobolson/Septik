#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Range {
    pub value: u8,
}
impl Range {
    pub const max: u8 = 255;
    pub const min: u8 = 0;

    pub fn map<T>(value: T, min: T, max: T) -> Self {
        unimplemented!();

        let v = Self::min;

        return Self { value: v };
    }
}
