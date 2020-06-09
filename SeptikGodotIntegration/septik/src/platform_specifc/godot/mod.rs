mod input_handler;
use crate::lib_core;
use lib_core::{EngineInputs, InputType};

mod converters;

use crate::ecs;

use fixed;
use gdnative::*;

/// The HelloWorld "class"
#[derive(NativeClass)]
#[inherit(Node)]
pub struct GameEngine {
    frame: usize,
    executed_frames: usize,
    player_inputs: Vec<InputType>,
    world: ecs::World,
}

#[methods]
impl GameEngine {
    /// The "constructor" of the class.
    fn _init(_owner: Node) -> Self {
        Self {
            frame: 0,
            executed_frames: 0,
            player_inputs: vec![],
            world: ecs::World::new(),
        }
    }

    // To make a method known to Godot, use the #[export] attribute.
    // In Godot, script "classes" do not actually inherit the parent class.
    // Instead, they are "attached" to the parent object, called the "owner".
    //
    // In order to enable access to the owner, it is passed as the second
    // argument to every single exposed method. As a result, all exposed
    // methods MUST have `owner: BaseClass` as their second arguments,
    // before all other arguments in the signature.
    #[export]
    fn _ready(&self, _owner: Node) {}

    #[export]
    unsafe fn _input(&mut self, mut owner: Node, event: InputEvent) {
        let i = input_handler::get_input_from_event(event);
        if i.is_some() {
            let i = i.unwrap();
            godot_print!("input: {:?}", i);

            self.player_inputs.push(i);
        }
    }

    #[export]
    unsafe fn _physics_process(&mut self, mut owner: Node, delta: f64) {
        self.frame += 1;

        self.player_inputs.append(&mut input_handler::input_poll());

        if self.player_inputs.is_empty() == false {
            self.executed_frames += 1;
            godot_print!("f: {}", self.executed_frames);

            self.world.register_player_inputs(&self.player_inputs);
            self.world.dispatch();

            self.player_inputs.clear();
        }

        self.link_to_gd_nodes(owner);
    }

    fn link_to_gd_nodes(&mut self, mut owner: Node) {
        // Create any nodes that need be created
        for e in self.world.entities() {
            let gdnode = &self.world.gd_nodes[e];
            let transform = &self.world.transforms[e];

            // Todo: what should constitute a gdnode creation?
            if transform.is_none() {
                continue;
            }

            //TODO: deletion of nodes
            //TODO: what happens when an entity is deleted?
            // Create gdnode if it doesn't exist
            if gdnode.is_none() {
                let mut gd_node = gdnative::Node2D::new();

                unsafe {
                    gd_node.add_child(Some(gdnative::Button::new().cast().unwrap()), false);
                }

                unsafe {
                    owner.add_child(Some(unsafe { gd_node.cast().unwrap() }), false);
                };

                let id = unsafe { gd_node.get_instance_id() };
                self.world.gd_nodes[e] = Some(ecs::components::GdNodeComponent::new(id));
            }
        }

        // Get the nodes
        let children = unsafe { owner.get_children() };
        let mut children: Vec<Node> = children
            .iter()
            .filter(|c| c.has_method(&GodotString::from_str("get_instance_id")))
            .map(|c| c.try_to_object::<Node>())
            .filter(|c| c.is_some())
            .map(|c| c.unwrap())
            .collect();

        // Update transforms
        for e in self.world.entities() {
            let gd_node_component = &self.world.gd_nodes[e];
            if gd_node_component.is_none() {
                continue;
            }

            let transform = &self.world.transforms[e];
            if transform.is_none() {
                continue;
            }

            let aabb = &self.world.aabbs[e];

            let transform = transform.as_ref().unwrap();

            let gd_node_component = gd_node_component.as_ref().unwrap();

            let mut gd_node = children
                .iter_mut()
                .find(|c| unsafe { c.get_instance_id() } == gd_node_component.id);

            if gd_node.is_none() {
                continue;
            }

            let gd_node = gd_node.unwrap();
            let gd_node = unsafe { gd_node.cast::<Node2D>() };
            if gd_node.is_some() {
                // Update position
                unsafe { gd_node.unwrap().set_position(transform.position.into()) };

                if aabb.is_some() {
                    let aabb = aabb.as_ref().unwrap().aabb;

                    let diff = aabb.max - aabb.min;
                    godot_print!("aabb:{:?}", diff);

                    unsafe {
                        gd_node.unwrap().set_scale(diff.into());
                    }
                }
            }
        }
    }
}

// Function that registers all exposed classes to Godot
fn init(handle: gdnative::init::InitHandle) {
    handle.add_class::<GameEngine>();
}

// macros that create the entry-points of the dynamic library.
godot_gdnative_init!();
godot_nativescript_init!(init);
godot_gdnative_terminate!();
