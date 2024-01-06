use std::collections::HashMap;
use benimator::{Animation, FrameRate};
use crate::animations::animation::LiegeAnimation;

pub enum GoblinAnimationStates {
    GoblinIdleRight,
    GoblinIdleLeft,
    GoblinDie,
    GoblinAttackRight,
    GoblinAttackLeft,
    GoblinAttackUpRight,
    GoblinAttackUpLeft,
    GoblinWalkRight,
    GoblinWalkLeft,
    GoblinWalkUpRight,
    GoblinWalkUpLeft
}

impl GoblinAnimationStates {
    pub fn to_string(&self) -> &str {
        match self {
            GoblinAnimationStates::GoblinIdleRight => "goblin_idle_right",
            GoblinAnimationStates::GoblinIdleLeft => "goblin_idle_left",
            GoblinAnimationStates::GoblinDie => "goblin_die",
            GoblinAnimationStates::GoblinAttackRight => "goblin_attack_right",
            GoblinAnimationStates::GoblinAttackLeft => "goblin_attack_left",
            GoblinAnimationStates::GoblinAttackUpRight => "goblin_attack_up_right",
            GoblinAnimationStates::GoblinAttackUpLeft => "goblin_attack_up_left",
            GoblinAnimationStates::GoblinWalkRight => "goblin_walk_right",
            GoblinAnimationStates::GoblinWalkLeft => "goblin_walk_left",
            GoblinAnimationStates::GoblinWalkUpRight => "goblin_walk_up_right",
            GoblinAnimationStates::GoblinWalkUpLeft => "goblin_walk_up_left"
        }
    }
}

pub fn load_goblin_animations() -> HashMap<String, LiegeAnimation> {
    // Load and deserialize our spritesheet information
    let file_content = match crate::animations::animation::read_json_file("resources/characters/goblin/goblin.json") {
        Ok(content) => content,
        Err(e) => panic!("Failed to read file: {}", e),
    };

    let sprite_sheet = match crate::animations::animation::deserialize_json(&file_content) {
        Ok(content) => content,
        Err(e) => panic!("Failed to deserialize: {}", e)
    };

    let mut animations_map: HashMap<String, LiegeAnimation> = HashMap::new();

    let goblin_idle_right = LiegeAnimation{
        frames: vec![
            sprite_sheet.frames["goblin_idle_right_1.png"],
            sprite_sheet.frames["goblin_idle_right_2.png"],
            sprite_sheet.frames["goblin_idle_right_3.png"],
            sprite_sheet.frames["goblin_idle_right_4.png"],
            sprite_sheet.frames["goblin_idle_right_5.png"],
            sprite_sheet.frames["goblin_idle_right_6.png"],
            sprite_sheet.frames["goblin_idle_right_7.png"],
            sprite_sheet.frames["goblin_idle_right_8.png"],
            sprite_sheet.frames["goblin_idle_right_9.png"],
            sprite_sheet.frames["goblin_idle_right_10.png"],
            sprite_sheet.frames["goblin_idle_right_11.png"],
            sprite_sheet.frames["goblin_idle_right_12.png"],
            sprite_sheet.frames["goblin_idle_right_13.png"],
            sprite_sheet.frames["goblin_idle_right_14.png"],
            sprite_sheet.frames["goblin_idle_right_15.png"],
            sprite_sheet.frames["goblin_idle_right_16.png"],
        ],
        animation: Animation::from_indices(0..15, FrameRate::from_fps(5.)),
    };

    let goblin_idle_left = LiegeAnimation{
        frames: vec![
            sprite_sheet.frames["goblin_idle_left_1.png"],
            sprite_sheet.frames["goblin_idle_left_2.png"],
            sprite_sheet.frames["goblin_idle_left_3.png"],
            sprite_sheet.frames["goblin_idle_left_4.png"],
            sprite_sheet.frames["goblin_idle_left_5.png"],
            sprite_sheet.frames["goblin_idle_left_6.png"],
            sprite_sheet.frames["goblin_idle_left_7.png"],
            sprite_sheet.frames["goblin_idle_left_8.png"],
            sprite_sheet.frames["goblin_idle_left_9.png"],
            sprite_sheet.frames["goblin_idle_left_10.png"],
            sprite_sheet.frames["goblin_idle_left_11.png"],
            sprite_sheet.frames["goblin_idle_left_12.png"],
            sprite_sheet.frames["goblin_idle_left_13.png"],
            sprite_sheet.frames["goblin_idle_left_14.png"],
            sprite_sheet.frames["goblin_idle_left_15.png"],
            sprite_sheet.frames["goblin_idle_left_16.png"],
        ],
        animation: Animation::from_indices(0..15, FrameRate::from_fps(5.)),
    };

    let goblin_walk_right = LiegeAnimation{
        frames: vec![
            sprite_sheet.frames["goblin_walk_right_1.png"],
            sprite_sheet.frames["goblin_walk_right_2.png"],
            sprite_sheet.frames["goblin_walk_right_3.png"],
            sprite_sheet.frames["goblin_walk_right_4.png"],

        ],
        animation: Animation::from_indices(0..3, FrameRate::from_fps(8.)),
    };
    let goblin_walk_left = LiegeAnimation{
        frames: vec![
            sprite_sheet.frames["goblin_walk_left_1.png"],
            sprite_sheet.frames["goblin_walk_left_2.png"],
            sprite_sheet.frames["goblin_walk_left_3.png"],
            sprite_sheet.frames["goblin_walk_left_4.png"],

        ],
        animation: Animation::from_indices(0..3, FrameRate::from_fps(8.)),
    };
    let goblin_walk_up_rightight = LiegeAnimation{
        frames: vec![
            sprite_sheet.frames["goblin_walk_up_right_1.png"],
            sprite_sheet.frames["goblin_walk_up_right_2.png"],
            sprite_sheet.frames["goblin_walk_up_right_3.png"],
            sprite_sheet.frames["goblin_walk_up_right_4.png"],

        ],
        animation: Animation::from_indices(0..3, FrameRate::from_fps(8.)),
    };
    let goblin_walk_up_left = LiegeAnimation{
        frames: vec![
            sprite_sheet.frames["goblin_walk_up_left_1.png"],
            sprite_sheet.frames["goblin_walk_up_left_2.png"],
            sprite_sheet.frames["goblin_walk_up_left_3.png"],
            sprite_sheet.frames["goblin_walk_up_left_4.png"],

        ],
        animation: Animation::from_indices(0..3, FrameRate::from_fps(8.)),
    };

    let goblin_attack_right = LiegeAnimation{
        frames: vec![
            sprite_sheet.frames["goblin_attack_right_1.png"],
            sprite_sheet.frames["goblin_attack_right_2.png"],
            sprite_sheet.frames["goblin_attack_right_3.png"],
            sprite_sheet.frames["goblin_attack_right_4.png"],

        ],
        animation: Animation::from_indices(0..3, FrameRate::from_fps(8.)),
    };

    let goblin_attack_up_right = LiegeAnimation{
        frames: vec![
            sprite_sheet.frames["goblin_attack_up_right_1.png"],
            sprite_sheet.frames["goblin_attack_up_right_2.png"],
            sprite_sheet.frames["goblin_attack_up_right_3.png"],
            sprite_sheet.frames["goblin_attack_up_right_4.png"],

        ],
        animation: Animation::from_indices(0..3, FrameRate::from_fps(8.)),
    };
    let goblin_attack_left = LiegeAnimation{
        frames: vec![
            sprite_sheet.frames["goblin_attack_left_1.png"],
            sprite_sheet.frames["goblin_attack_left_2.png"],
            sprite_sheet.frames["goblin_attack_left_3.png"],
            sprite_sheet.frames["goblin_attack_left_4.png"],

        ],
        animation: Animation::from_indices(0..3, FrameRate::from_fps(8.)),
    };
    let goblin_attack_up_left = LiegeAnimation{
        frames: vec![
            sprite_sheet.frames["goblin_attack_up_left_1.png"],
            sprite_sheet.frames["goblin_attack_up_left_2.png"],
            sprite_sheet.frames["goblin_attack_up_left_3.png"],
            sprite_sheet.frames["goblin_attack_up_left_4.png"],

        ],
        animation: Animation::from_indices(0..3, FrameRate::from_fps(8.)),
    };

    let goblin_die = LiegeAnimation{
        frames: vec![
            sprite_sheet.frames["goblin_die_1.png"],
            sprite_sheet.frames["goblin_die_2.png"],
            sprite_sheet.frames["goblin_die_3.png"],
            sprite_sheet.frames["goblin_die_4.png"],
            sprite_sheet.frames["goblin_die_5.png"],
            sprite_sheet.frames["goblin_die_6.png"],
            sprite_sheet.frames["goblin_die_7.png"],
            sprite_sheet.frames["goblin_die_8.png"],
            sprite_sheet.frames["goblin_die_9.png"],
            sprite_sheet.frames["goblin_die_10.png"],
            sprite_sheet.frames["goblin_die_11.png"],
            sprite_sheet.frames["goblin_die_12.png"],

        ],
        animation: Animation::from_indices(0..11, FrameRate::from_fps(8.)),
    };

    animations_map.insert("goblin_idle_right".to_string(), goblin_idle_right);
    animations_map.insert("goblin_idle_left".to_string(), goblin_idle_left);
    animations_map.insert("goblin_walk_right".to_string(), goblin_walk_right);
    animations_map.insert("goblin_walk_left".to_string(), goblin_walk_left);
    animations_map.insert("goblin_walk_up_right".to_string(), goblin_walk_up_rightight);
    animations_map.insert("goblin_walk_up_left".to_string(), goblin_walk_up_left);
    animations_map.insert("goblin_attack_right".to_string(), goblin_attack_right);
    animations_map.insert("goblin_attack_up_right".to_string(), goblin_attack_up_right);
    animations_map.insert("goblin_attack_left".to_string(), goblin_attack_left);
    animations_map.insert("goblin_attack_up_left".to_string(), goblin_attack_up_left);
    animations_map.insert("goblin_die".to_string(), goblin_die);

    animations_map
}