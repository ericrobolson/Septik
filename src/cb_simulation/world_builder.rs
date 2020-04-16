extern crate specs;
use specs::prelude::*;

use super::*;

use components;
use components::{
    actor_components, audio, editor_components, gfx_components, ik_components, menu_components,
    physics_components, voxel_components, ComponentLinker,
};

pub fn new(mode: CbSimulationModes) -> specs::World {
    let mut world = World::new();

    // Physics components
    physics_components::PhysicsComponentsLinker::register_components(&mut world);
    // Character components
    character_components::ComponentsLinker::register_components(&mut world);
    // GFX components
    gfx_components::GfxComponentsLinker::register_components(&mut world);
    // IK components
    ik_components::IkComponentsLinker::register_components(&mut world);
    // Actor components
    actor_components::ActorComponentsLinker::register_components(&mut world);
    // Voxel components
    voxel_components::VoxelComponentsLinker::register_components(&mut world);
    // Editor components
    editor_components::EditorComponentsLinker::register_components(&mut world);

    // Menu components
    menu_components::MenuComponentsLinker::register_components(&mut world);

    // Audio components

    // Resources
    {
        world.insert(CbSystemValues::new());
    }

    // Setup entities
    {
        if mode == CbSimulationModes::RtsMode {
            assemblages::rts_assemblages::new_unit(&mut world)
        }
    }

    return world;
}
