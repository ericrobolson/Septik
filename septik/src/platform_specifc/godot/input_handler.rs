use super::*;
use lib_core::{EngineInputs, InputType};

struct InputConst {}
impl InputConst {
    pub const MOVE_FORWARD: &'static str = "character_move_up";
    pub const MOVE_BACK: &'static str = "character_move_down";
    pub const MOVE_LEFT: &'static str = "character_move_left";
    pub const MOVE_RIGHT: &'static str = "character_move_right";

    pub const JUMP: &'static str = "character_jump";
    pub const DODGE: &'static str = "character_dodge";

    pub const HORZ_ATK: &'static str = "character_horizontal_attack";
    pub const VERT_ATK: &'static str = "character_vertical_attack";

    fn inputs() -> Vec<(&'static str, EngineInputs)> {
        return vec![
            (Self::MOVE_FORWARD, EngineInputs::MoveForward),
            (Self::MOVE_BACK, EngineInputs::MoveBack),
            (Self::MOVE_LEFT, EngineInputs::MoveLeft),
            (Self::MOVE_RIGHT, EngineInputs::MoveRight),
        ];
    }
}

pub fn get_input_from_event(event: InputEvent) -> Option<InputType> {
    for (input_str, engine_input) in InputConst::inputs() {
        let i = input_from_event(input_str, engine_input, &event);
        if i.is_some() {
            return i;
        }
    }

    return None;
}

fn input_from_event(
    button: &'static str,
    input: EngineInputs,
    event: &InputEvent,
) -> Option<InputType> {
    let local_player_id = 0; //TODO!!

    if event.is_action_pressed(GodotString::from_str(button), false) {
        return Some(InputType::Pressed(local_player_id, input));
    } else if event.is_action_released(GodotString::from_str(button)) {
        return Some(InputType::Released(local_player_id, input));
    }

    return None;
}

pub fn input_poll() -> Vec<InputType> {
    let mut inputs = vec![];

    let local_player_id = 0; //TODO!!

    let input = Input::godot_singleton();
    if Input::is_action_pressed(&input, GodotString::from_str(InputConst::MOVE_FORWARD)) {
        inputs.push(InputType::Held(local_player_id, EngineInputs::MoveForward));
    }
    if Input::is_action_pressed(&input, GodotString::from_str(InputConst::MOVE_BACK)) {
        inputs.push(InputType::Held(local_player_id, EngineInputs::MoveBack));
    }
    if Input::is_action_pressed(&input, GodotString::from_str(InputConst::MOVE_LEFT)) {
        inputs.push(InputType::Held(local_player_id, EngineInputs::MoveLeft));
    }
    if Input::is_action_pressed(&input, GodotString::from_str(InputConst::MOVE_RIGHT)) {
        inputs.push(InputType::Held(local_player_id, EngineInputs::MoveRight));
    }

    /*
    if Input::is_action_pressed(&input, GodotString::from_str(InputConst::HORZ_ATK)) {
        inputs.push(InputType::Held(EngineInputs::HorizontalAttack));
    }
    if Input::is_action_pressed(&input, GodotString::from_str(InputConst::VERT_ATK)) {
        inputs.push(InputType::Held(EngineInputs::VerticalAttack));
    }
    */
    if Input::is_action_pressed(&input, GodotString::from_str(InputConst::JUMP)) {
        inputs.push(InputType::Held(local_player_id, EngineInputs::Jump));
    }
    if Input::is_action_pressed(&input, GodotString::from_str(InputConst::DODGE)) {
        inputs.push(InputType::Held(local_player_id, EngineInputs::Dodge));
    }

    return inputs;
}
