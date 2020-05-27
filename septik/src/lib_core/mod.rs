mod aabb;
pub mod colors;
pub mod math;
pub mod voxels;

pub use aabb::Aabb;

mod direction;
pub use direction::Direction;

pub type PlayerId = u8;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum InputType {
    Pressed(PlayerId, EngineInputs),
    Held(PlayerId, EngineInputs),
    Released(PlayerId, EngineInputs),
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum EngineInputs {
    MoveUp,
    MoveDown,
    MoveLeft,
    MoveRight,

    HorizontalAttack,
    VerticalAttack,

    Jump,
    Dodge,
}
