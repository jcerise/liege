use std::collections::HashMap;
use benimator::{Animation, FrameRate};
use crate::animations::animation::LiegeAnimation;

pub enum RogueAnimationStates {
    RogueIdleRight,
    RogueIdleLeft,
    RogueDie,
    RogueAttackRight,
    RogueAttackLeft,
    RogueAttackUpRight,
    RogueAttackUpLeft,
    RogueWalkRight,
    RogueWalkLeft,
    RogueWalkUpRight,
    RogueWalkUpLeft
}

impl RogueAnimationStates {
    pub fn to_string(&self) -> &str {
        match self {
            RogueAnimationStates::RogueIdleRight => "rogue_idle_right",
            RogueAnimationStates::RogueIdleLeft => "rogue_idle_left",
            RogueAnimationStates::RogueDie => "rogue_die",
            RogueAnimationStates::RogueAttackRight => "rogue_attack_right",
            RogueAnimationStates::RogueAttackLeft => "rogue_attack_left",
            RogueAnimationStates::RogueAttackUpRight => "rogue_attack_up_right",
            RogueAnimationStates::RogueAttackUpLeft => "rogue_attack_up_left",
            RogueAnimationStates::RogueWalkRight => "rogue_walk_right",
            RogueAnimationStates::RogueWalkLeft => "rogue_walk_left",
            RogueAnimationStates::RogueWalkUpRight => "rogue_walk_up_right",
            RogueAnimationStates::RogueWalkUpLeft => "rogue_walk_up_left"
        }
    }
}

pub fn load_rogue_animations() -> HashMap<String, LiegeAnimation> {
    // Load and deserialize our spritesheet information
    let file_content = match crate::animations::animation::read_json_file("resources/characters/rogue/rogue.json") {
        Ok(content) => content,
        Err(e) => panic!("Failed to read file: {}", e),
    };

    let sprite_sheet = match crate::animations::animation::deserialize_json(&file_content) {
        Ok(content) => content,
        Err(e) => panic!("Failed to deserialize: {}", e)
    };

    let mut animations_map: HashMap<String, LiegeAnimation> = HashMap::new();

    let rogue_idle_right = LiegeAnimation{
        frames: vec![
            sprite_sheet.frames["rogue_idle_right_1.png"],
            sprite_sheet.frames["rogue_idle_right_2.png"],
            sprite_sheet.frames["rogue_idle_right_3.png"],
            sprite_sheet.frames["rogue_idle_right_4.png"],
            sprite_sheet.frames["rogue_idle_right_5.png"],
            sprite_sheet.frames["rogue_idle_right_6.png"],
            sprite_sheet.frames["rogue_idle_right_7.png"],
            sprite_sheet.frames["rogue_idle_right_8.png"],
            sprite_sheet.frames["rogue_idle_right_9.png"],
            sprite_sheet.frames["rogue_idle_right_10.png"],
            sprite_sheet.frames["rogue_idle_right_11.png"],
            sprite_sheet.frames["rogue_idle_right_12.png"],
            sprite_sheet.frames["rogue_idle_right_13.png"],
            sprite_sheet.frames["rogue_idle_right_14.png"],
            sprite_sheet.frames["rogue_idle_right_15.png"],
            sprite_sheet.frames["rogue_idle_right_16.png"],
        ],
        animation: Animation::from_indices(0..15, FrameRate::from_fps(5.)),
    };

    let rogue_idle_left = LiegeAnimation{
        frames: vec![
            sprite_sheet.frames["rogue_idle_left_1.png"],
            sprite_sheet.frames["rogue_idle_left_2.png"],
            sprite_sheet.frames["rogue_idle_left_3.png"],
            sprite_sheet.frames["rogue_idle_left_4.png"],
            sprite_sheet.frames["rogue_idle_left_5.png"],
            sprite_sheet.frames["rogue_idle_left_6.png"],
            sprite_sheet.frames["rogue_idle_left_7.png"],
            sprite_sheet.frames["rogue_idle_left_8.png"],
            sprite_sheet.frames["rogue_idle_left_9.png"],
            sprite_sheet.frames["rogue_idle_left_10.png"],
            sprite_sheet.frames["rogue_idle_left_11.png"],
            sprite_sheet.frames["rogue_idle_left_12.png"],
            sprite_sheet.frames["rogue_idle_left_13.png"],
            sprite_sheet.frames["rogue_idle_left_14.png"],
            sprite_sheet.frames["rogue_idle_left_15.png"],
            sprite_sheet.frames["rogue_idle_left_16.png"],
        ],
        animation: Animation::from_indices(0..15, FrameRate::from_fps(5.)),
    };

    let rogue_attack_right = LiegeAnimation{
        frames: vec![
            sprite_sheet.frames["rogue_attack_right_1.png"],
            sprite_sheet.frames["rogue_attack_right_2.png"],
            sprite_sheet.frames["rogue_attack_right_3.png"],
            sprite_sheet.frames["rogue_attack_right_4.png"],

        ],
        animation: Animation::from_indices(0..3, FrameRate::from_fps(8.)),
    };
    let rogue_attack_left = LiegeAnimation{
        frames: vec![
            sprite_sheet.frames["rogue_attack_left_1.png"],
            sprite_sheet.frames["rogue_attack_left_2.png"],
            sprite_sheet.frames["rogue_attack_left_3.png"],
            sprite_sheet.frames["rogue_attack_left_4.png"],

        ],
        animation: Animation::from_indices(0..3, FrameRate::from_fps(8.)),
    };
    let rogue_attack_up_right = LiegeAnimation{
        frames: vec![
            sprite_sheet.frames["rogue_attack_up_right_1.png"],
            sprite_sheet.frames["rogue_attack_up_right_2.png"],
            sprite_sheet.frames["rogue_attack_up_right_3.png"],
            sprite_sheet.frames["rogue_attack_up_right_4.png"],

        ],
        animation: Animation::from_indices(0..3, FrameRate::from_fps(8.)),
    };
    let rogue_attack_up_left = LiegeAnimation{
        frames: vec![
            sprite_sheet.frames["rogue_attack_up_left_1.png"],
            sprite_sheet.frames["rogue_attack_up_left_2.png"],
            sprite_sheet.frames["rogue_attack_up_left_3.png"],
            sprite_sheet.frames["rogue_attack_up_left_4.png"],

        ],
        animation: Animation::from_indices(0..3, FrameRate::from_fps(8.)),
    };

    let rogue_walk_right = LiegeAnimation{
        frames: vec![
            sprite_sheet.frames["rogue_walk_r_1.png"],
            sprite_sheet.frames["rogue_walk_r_2.png"],
            sprite_sheet.frames["rogue_walk_r_3.png"],
            sprite_sheet.frames["rogue_walk_r_4.png"],

        ],
        animation: Animation::from_indices(0..3, FrameRate::from_fps(8.)),
    };
    let rogue_walk_left = LiegeAnimation{
        frames: vec![
            sprite_sheet.frames["rogue_walk_l_1.png"],
            sprite_sheet.frames["rogue_walk_l_2.png"],
            sprite_sheet.frames["rogue_walk_l_3.png"],
            sprite_sheet.frames["rogue_walk_l_4.png"],

        ],
        animation: Animation::from_indices(0..3, FrameRate::from_fps(8.)),
    };
    let rogue_walk_up_right = LiegeAnimation{
        frames: vec![
            sprite_sheet.frames["rogue_walk_up_r_1.png"],
            sprite_sheet.frames["rogue_walk_up_r_2.png"],
            sprite_sheet.frames["rogue_walk_up_r_3.png"],
            sprite_sheet.frames["rogue_walk_up_r_4.png"],

        ],
        animation: Animation::from_indices(0..3, FrameRate::from_fps(8.)),
    };
    let rogue_walk_up_left = LiegeAnimation{
        frames: vec![
            sprite_sheet.frames["rogue_walk_up_l_1.png"],
            sprite_sheet.frames["rogue_walk_up_l_2.png"],
            sprite_sheet.frames["rogue_walk_up_l_3.png"],
            sprite_sheet.frames["rogue_walk_up_l_4.png"],

        ],
        animation: Animation::from_indices(0..3, FrameRate::from_fps(8.)),
    };

    let rogue_die = LiegeAnimation{
        frames: vec![
            sprite_sheet.frames["rogue_die_1.png"],
            sprite_sheet.frames["rogue_die_2.png"],
            sprite_sheet.frames["rogue_die_3.png"],
            sprite_sheet.frames["rogue_die_4.png"],
            sprite_sheet.frames["rogue_die_5.png"],
            sprite_sheet.frames["rogue_die_6.png"],
            sprite_sheet.frames["rogue_die_7.png"],
            sprite_sheet.frames["rogue_die_8.png"],
            sprite_sheet.frames["rogue_die_9.png"],
            sprite_sheet.frames["rogue_die_10.png"],
            sprite_sheet.frames["rogue_die_11.png"],
            sprite_sheet.frames["rogue_die_12.png"],
            sprite_sheet.frames["rogue_die_13.png"],
            sprite_sheet.frames["rogue_die_14.png"],
            sprite_sheet.frames["rogue_die_15.png"],
            sprite_sheet.frames["rogue_die_16.png"],
            sprite_sheet.frames["rogue_die_17.png"],
            sprite_sheet.frames["rogue_die_18.png"],
            sprite_sheet.frames["rogue_die_19.png"],
            sprite_sheet.frames["rogue_die_20.png"],
            sprite_sheet.frames["rogue_die_21.png"],
            sprite_sheet.frames["rogue_die_22.png"],
            sprite_sheet.frames["rogue_die_23.png"],
            sprite_sheet.frames["rogue_die_24.png"],
            sprite_sheet.frames["rogue_die_25.png"],
            sprite_sheet.frames["rogue_die_26.png"],

        ],
        animation: Animation::from_indices(0..25, FrameRate::from_fps(8.)).once(),
    };

    animations_map.insert("rogue_idle_right".to_string(), rogue_idle_right);
    animations_map.insert("rogue_idle_left".to_string(), rogue_idle_left);
    animations_map.insert("rogue_die".to_string(), rogue_die);
    animations_map.insert("rogue_attack_right".to_string(), rogue_attack_right);
    animations_map.insert("rogue_attack_left".to_string(), rogue_attack_left);
    animations_map.insert("rogue_attack_up_right".to_string(), rogue_attack_up_right);
    animations_map.insert("rogue_attack_up_left".to_string(), rogue_attack_up_left);
    animations_map.insert("rogue_walk_right".to_string(), rogue_walk_right);
    animations_map.insert("rogue_walk_left".to_string(), rogue_walk_left);
    animations_map.insert("rogue_walk_up_right".to_string(), rogue_walk_up_right);
    animations_map.insert("rogue_walk_up_left".to_string(), rogue_walk_up_left);

    animations_map
}