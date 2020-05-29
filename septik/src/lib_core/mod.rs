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
    /// Cursor movement normalized to (-1,-1,0) to (1,1,0), where (0,0,0) is the center of the screen.
    CursorNormalized(PlayerId, math::Vec3d),
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum EngineInputs {
    MoveForward,
    MoveBack,
    MoveLeft,
    MoveRight,

    HorizontalAttack,
    VerticalAttack,

    Jump,
    Crouch,
    Dodge,
}
