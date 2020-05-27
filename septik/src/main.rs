pub mod ecs;
pub mod lib_core;
pub mod platform_specifc;

fn main() {
    let mut win_gfx = platform_specifc::WindowGfxBuilder::build();

    let mut world = ecs::World::new();

    let chunk = lib_core::voxels::voxel_chunk::VoxelChunk::new();

    loop {
        world.register_player_inputs(&win_gfx.poll_input());
        world.dispatch();
        win_gfx.render(&world);
    }
}
