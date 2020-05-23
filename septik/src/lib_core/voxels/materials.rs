pub enum VoxelMaterials {
    Dirt,
    Grass,
    Water,
    Rock,
}

impl VoxelMaterials {
    pub fn to_base_type(&self) -> VoxelMaterialType {
        match self {
            Self::Dirt => 0,
            Self::Grass => 1,
            Self::Water => 2,
            Self::Rock => 3,
        }
    }

    pub fn from_base_type(mat: VoxelMaterialType) -> Option<Self> {
        match mat {
            0 => Some(Self::Dirt),
            1 => Some(Self::Grass),
            2 => Some(Self::Water),
            3 => Some(Self::Rock),
            _ => None,
        }
    }
}

pub type VoxelMaterialType = u8;
