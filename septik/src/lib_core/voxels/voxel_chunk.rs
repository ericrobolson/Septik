use super::voxel::Voxel;
use crate::lib_core::colors::*;

pub struct VoxelChunk {
    pub palatte: Palatte,
    pub x_voxels: [Voxel; Self::BASE_ARRAY_SIZE],
    pub y_voxels: [Voxel; Self::BASE_ARRAY_SIZE],
    pub z_voxels: [Voxel; Self::BASE_ARRAY_SIZE],
}

impl VoxelChunk {
    /// The size of the array of voxels just in this chunk
    pub const BASE_ARRAY_SIZE: usize = 32;
    /// The total size of the array
    pub const TOTAL_ARRAY_SIZE: usize = Self::BASE_ARRAY_SIZE;
}
