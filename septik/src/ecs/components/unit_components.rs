use crate::lib_core::math::{FixedNumber, Range, Rotation3d, Vec3d};

/// Base struct for unit data
pub struct UnitComponent {
    /// The modifier for melee attack hits
    pub weapon_skill: u8,
    /// The modifier for ranged attack hits
    pub ballistic_skill: u8,
    /// The modifier for base melee strength
    pub melee_str: u8,
    /// The modifier for calculating the number to trigger a wound
    pub toughness: u8,
    /// The remaining wounds a character has left
    pub wounds: u8,
    /// The order that the unit executes their actions
    pub initiative: u8,
    /// When a wound is triggered, use this stat to determine whether the wound actually hits
    pub armor_save: u8,
    /// The base number of melee attacks a character has
    pub melee_atks: u8,
    /// The base size of the unit
    pub base_size: u8,
}

impl UnitComponent {
    /// Initialize a basic unit with minimum stats
    pub fn default() -> Self {
        return Self {
            weapon_skill: 1,
            ballistic_skill: 1,
            melee_str: 1,
            toughness: 1,
            wounds: 1,
            initiative: 1,
            armor_save: 1,
            melee_atks: 1,
            base_size: 1,
        };
    }
}
