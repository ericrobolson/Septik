use super::{materials::*, *};
use crate::lib_core::colors::*;

pub struct Voxel {
    pub palatte_index: PalatteIndexType,
    pub material_type: VoxelMaterialType,
    pub active: bool,
}

impl Voxel {
    pub fn is_active(&self) -> bool {
        unimplemented!();
    }
}
