use super::{materials::*, *};
use crate::lib_core::colors::*;

#[derive(Copy, Clone, Debug)]
pub struct Voxel {
    pub palatte_index: PalatteIndexType,
    pub material_type: VoxelMaterialType,
    pub active: bool,
}

impl Voxel {
    pub fn is_active(&self) -> bool {
        self.active
    }

    pub fn new() -> Self {
        Self {
            palatte_index: 0,
            material_type: 0,
            active: true,
        }
    }
}
