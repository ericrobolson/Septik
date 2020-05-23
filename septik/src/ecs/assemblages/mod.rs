use crate::ecs::{components, World};
use crate::lib_core::{
    math::{FixedNumber, Range, Vec3d},
    Aabb,
};

pub fn assemblage_player(world: &mut World) {
    let e = world.add_entity();

    world.players[e] = Some(components::PlayerComponent::new());
    world.engine_inputs[e] = Some(components::EngineInputsComponent::new());
    world.transforms[e] = Some(components::TransformComponent::new());
    world.velocities[e] = Some(components::VelocityComponent::new());
    world.move_speeds[e] = Some(components::MoveSpeedComponent::new(8.into()));

    let aabb_size = 16;
    let aabb = Aabb::new(
        (-aabb_size, -aabb_size, -aabb_size).into(),
        (aabb_size, aabb_size, aabb_size).into(),
    );

    world.aabbs[e] = Some(components::AabbComponent::new(aabb));
}

pub fn assemblage_basic_enemy(world: &mut World) {
    let e = world.add_entity();

    world.enemies[e] = Some(components::EnemyComponent::new());
    world.ais[e] = Some(components::AiComponent::new());
    world.engine_inputs[e] = Some(components::EngineInputsComponent::new());
    let mut transform = components::TransformComponent::new();
    transform.position += (100, 100, 0).into();

    world.transforms[e] = Some(transform);
    world.velocities[e] = Some(components::VelocityComponent::new());
    world.move_speeds[e] = Some(components::MoveSpeedComponent::new(2.into()));
}
