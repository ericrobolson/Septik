use super::voxel::Voxel;
use crate::lib_core::colors::*;

#[derive(Clone)]
pub struct VoxelChunk {
    pub palatte: Palatte,
    pub voxels: [[[Voxel; Self::BASE_ARRAY_SIZE]; Self::BASE_ARRAY_SIZE]; Self::BASE_ARRAY_SIZE],
}

impl VoxelChunk {
    /// The size of the array of voxels in this chunk
    pub const BASE_ARRAY_SIZE: usize = 8;
    pub fn new() -> Self {
        Self {
            palatte: Palatte::new(),
            voxels: [[generate_voxel_row(); Self::BASE_ARRAY_SIZE]; Self::BASE_ARRAY_SIZE],
        }
    }
}

fn generate_voxel_row() -> [Voxel; VoxelChunk::BASE_ARRAY_SIZE] {
    [Voxel::new(); VoxelChunk::BASE_ARRAY_SIZE]
}
