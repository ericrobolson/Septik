mod assemblages;
pub mod systems;

use crate::lib_core::{EngineInputs, InputType};

pub mod components;
use components::{
    AabbComponent, AiComponent, AilmentsComponent, EnemyComponent, EngineInputsComponent,
    FacingComponent, GdNodeComponent, HitPointComponent, MoveSpeedComponent, PlayerComponent,
    TargetComponent, TargetableComponent, TransformComponent, VelocityComponent,
    VoxelChunkComponent,
};

pub type Entity = usize;

pub type Storage<T> = Vec<Option<T>>;

// TODO: parent/child implementation based off of this:
// http://bitsquid.blogspot.com/2014/10/building-data-oriented-entity-system.html
pub struct World {
    next_entity: Entity,
    pub parents: Storage<Entity>,
    pub ailments: Storage<AilmentsComponent>,
    pub engine_inputs: Storage<EngineInputsComponent>,
    pub facing_direction: Storage<FacingComponent>,
    pub hitpoints: Storage<HitPointComponent>,
    pub players: Storage<PlayerComponent>,
    pub transforms: Storage<TransformComponent>,
    pub velocities: Storage<VelocityComponent>,
    pub move_speeds: Storage<MoveSpeedComponent>,
    pub gd_nodes: Storage<GdNodeComponent>,
    pub targets: Storage<TargetComponent>,
    pub targetables: Storage<TargetableComponent>,
    pub ais: Storage<AiComponent>,
    pub enemies: Storage<EnemyComponent>,
    pub aabbs: Storage<AabbComponent>,
    pub voxel_chunks: Storage<VoxelChunkComponent>,
}

impl World {
    pub const MAX_ENTITIES: usize = 1000;
    pub fn new() -> Self {
        let mut world = Self {
            next_entity: 0,
            parents: generate_storage(),
            ailments: generate_storage(),
            engine_inputs: generate_storage(),
            facing_direction: generate_storage(),
            hitpoints: generate_storage(),
            players: generate_storage(),
            transforms: generate_storage(),
            velocities: generate_storage(),
            move_speeds: generate_storage(),
            gd_nodes: generate_storage(),
            targets: generate_storage(),
            targetables: generate_storage(),
            ais: generate_storage(),
            enemies: generate_storage(),
            aabbs: generate_storage(),
            voxel_chunks: generate_storage(),
        };

        assemblages::assemblage_player(&mut world);
        assemblages::assemblage_basic_enemy(&mut world);
        assemblages::assemblage_basic_voxel_chunk(&mut world);

        return world;
    }

    pub fn entities(&self) -> std::ops::Range<usize> {
        0..self.next_entity
    }

    pub fn register_player_inputs(&mut self, inputs: &Vec<InputType>) {
        for e in self.entities() {
            let player = &self.players[e];
            let engine_inputs = &self.engine_inputs[e];

            if player.is_none() || engine_inputs.is_none() {
                continue;
            }

            let mut engine_inputs = engine_inputs.clone().unwrap();
            engine_inputs.inputs.append(&mut (inputs.clone()));

            self.engine_inputs[e] = Some(engine_inputs);
        }
    }

    pub fn dispatch(&mut self) {
        systems::character_action_system(self);
        systems::position_update_system(self);

        self.maintain();
    }

    pub fn add_entity(&mut self) -> Entity {
        let e = self.next_entity;

        self.next_entity += 1;

        return e;
    }

    fn maintain(&mut self) {
        //TODO: shift entities over, delete any missing entities, update indexes.
        // General cleanup function.
        systems::input_cleanup_system(self);
    }

    fn delete_entity(&mut self, entity: Entity) {
        unimplemented!();
    }
}

fn generate_storage<TComponent>() -> Storage<TComponent> {
    let mut v = Vec::<Option<TComponent>>::with_capacity(World::MAX_ENTITIES);

    for _ in 0..World::MAX_ENTITIES {
        v.push(None);
    }

    return v;
}
