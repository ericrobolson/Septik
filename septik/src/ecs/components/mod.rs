use crate::ecs::Entity;
use crate::lib_core::{
    math::{FixedNumber, Range, Rotation3d, Vec3d},
    voxels::voxel_chunk::VoxelChunk,
    Aabb, Direction, InputType,
};

pub mod mesh_component;
pub use mesh_component::MeshComponent;

#[derive(Clone)]
pub struct ThirdPersonCameraComponent {
    pub relative_position: Vec3d,
    default_relative_position: Vec3d,
    pub rotation: Rotation3d,
}

impl ThirdPersonCameraComponent {
    pub fn new() -> Self {
        let default_pos = Vec3d::new((0).into(), 0.into(), (-10).into());
        return Self {
            relative_position: default_pos,
            default_relative_position: default_pos,
            rotation: Rotation3d::default(),
        };
    }
}

#[derive(Clone)]
pub struct VoxelChunkComponent {
    pub chunk: VoxelChunk,
}

impl VoxelChunkComponent {
    pub fn new() -> Self {
        Self {
            chunk: VoxelChunk::new(),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct TargetComponent {
    pub entity: Entity,
}

#[derive(Clone, Debug, PartialEq)]
pub struct AabbComponent {
    pub aabb: Aabb,
}

impl AabbComponent {
    pub fn new(aabb: Aabb) -> Self {
        Self { aabb: aabb }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct AiComponent {}

impl AiComponent {
    pub fn new() -> Self {
        return Self {};
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct EnemyComponent {}

impl EnemyComponent {
    pub fn new() -> Self {
        return Self {};
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct TargetableComponent {}

#[derive(Clone, Debug, PartialEq)]
pub struct GdNodeComponent {
    pub id: i64,
}

impl GdNodeComponent {
    pub fn new(id: i64) -> Self {
        return Self { id: id };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct MoveSpeedComponent {
    pub value: FixedNumber,
}

impl MoveSpeedComponent {
    pub fn new(value: FixedNumber) -> Self {
        return Self { value: value };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct HitPointComponent {
    pub value: u32,
    pub max_value: u32,
}

#[derive(Clone, Debug, PartialEq)]
pub struct VelocityComponent {
    pub value: Vec3d,
    pub rotational_velocity: Rotation3d,
}

impl VelocityComponent {
    pub fn new() -> Self {
        return Self {
            value: Vec3d::default(),
            rotational_velocity: Rotation3d::default(),
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct TransformComponent {
    pub position: Vec3d,
    pub rotation: Rotation3d,
}

impl TransformComponent {
    pub fn new() -> Self {
        return Self {
            position: Vec3d::default(),
            rotation: Rotation3d::default(),
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct PlayerComponent {}

impl PlayerComponent {
    pub fn new() -> Self {
        return Self {};
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct FacingComponent {
    pub direction: Direction,
}

impl FacingComponent {
    pub fn new(direction: Direction) -> Self {
        return Self {
            direction: direction,
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct EngineInputsComponent {
    pub inputs: Vec<InputType>,
}

impl EngineInputsComponent {
    pub fn new() -> Self {
        return Self { inputs: vec![] };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct AilmentsComponent {
    pub ailments: Vec<Ailment>,
}

#[derive(Clone, Debug, PartialEq)]
/// Information related to an ailment a character is inflicted with.
pub struct Ailment {
    /// The type of the modifier
    pub modifier_type: AilmentTypes,
    /// The multiplier of the modifier
    pub modifier_multiplier: u8,
    /// The current meter of the multiplier. Causes the effect to be triggered when full.
    pub status_meter: Range,
}

#[derive(Clone, Debug, PartialEq)]
/// Various types of ailments that can be inflicted upon characters. For now, keep small until more things need to be added.
pub enum AilmentTypes {
    /// A modifier which when triggered, causes a single hp loss event and resets the meter to 0.
    Bleed,
    /// A modifier which when triggered, will cause small HP loss until meter is drained completely.
    Poison,
    /// A modifier which when triggered, will instantly kill the character.
    Curse,
    /// A modifier which when triggered, causes all attacks and movement to increase in time to execute.
    Sloth,
    /// A modifier which when triggered, doubles the meter of any current ailments.
    BrainWorms,
}

impl AilmentTypes {
    pub const size: usize = 5;

    pub fn index(&self) -> usize {
        let index = match self {
            AilmentTypes::Bleed => 0,
            AilmentTypes::Poison => 1,
            AilmentTypes::Curse => 2,
            AilmentTypes::Sloth => 3,
            AilmentTypes::BrainWorms => 4,
        };

        return index;
    }

    pub fn display_name(&self) -> String {
        let name = match self {
            AilmentTypes::Bleed => String::from("Bleed"),
            AilmentTypes::Poison => String::from("Poison"),
            AilmentTypes::Curse => String::from("Curse"),
            AilmentTypes::Sloth => String::from("Sloth"),
            AilmentTypes::BrainWorms => String::from("Brainworms"),
        };

        return name.to_uppercase();
    }

    pub fn description(&self) -> String {
        let description = match self {
            AilmentTypes::Bleed => String::from("causes a single loss of HP."),
            AilmentTypes::Poison => String::from("causes HP loss until meter runs out."),
            AilmentTypes::Curse => String::from("causes instant death."),
            AilmentTypes::Sloth => String::from("slows down character movement and attacks."),
            AilmentTypes::BrainWorms => String::from("doubles the meter of all ailments."),
        };

        return format!("{}: When triggered, {}", self.display_name(), description);
    }
}
