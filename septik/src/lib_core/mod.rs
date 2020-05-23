mod aabb;
pub mod colors;
pub mod math;
pub mod voxels;

pub use aabb::Aabb;

mod direction;
pub use direction::Direction;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum InputType {
    Pressed(EngineInputs),
    Held(EngineInputs),
    Released(EngineInputs),
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
