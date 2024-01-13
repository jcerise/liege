use std::collections::HashMap;
use std::fs;
use benimator::{Animation, State};
use macroquad::math::Vec2;
use serde::{Deserialize, Serialize};

pub enum AnimationStates {
    IdleRight,
    IdleLeft,
    Die,
    AttackRight,
    AttackLeft,
    AttackUpRight,
    AttackUpLeft,
    WalkRight,
    WalkLeft,
    WalkUpRight,
    WalkUpLeft
}

impl AnimationStates {
    pub fn to_str(&self) -> &str {
        match self {
            AnimationStates::IdleRight => "_idle_right",
            AnimationStates::IdleLeft => "_idle_left",
            AnimationStates::Die => "_die",
            AnimationStates::AttackRight => "_attack_right",
            AnimationStates::AttackLeft => "_attack_left",
            AnimationStates::AttackUpRight => "_attack_up_right",
            AnimationStates::AttackUpLeft => "_attack_up_left",
            AnimationStates::WalkRight => "_walk_right",
            AnimationStates::WalkLeft => "_walk_left",
            AnimationStates::WalkUpRight => "_walk_up_right",
            AnimationStates::WalkUpLeft => "_walk_up_left"
        }
    }

    pub fn get_creature_animation(&self, creature_type: &str) -> String {
        format!("{}{}", creature_type, self.to_str())
    }
}

pub struct AnimationMap {
    pub animations: HashMap<String, LiegeAnimation>
}

pub struct UIAnimationMap<'a> {
    pub animations: HashMap<String, LiegeUIAnimation<'a>>
}

#[derive(Serialize, Deserialize, Clone, Copy, PartialEq, Debug)]
pub struct Frame {
    pub x: i32,
    pub y: i32,
    pub w: i32,
    pub h: i32,
}

#[derive(Serialize, Deserialize, Clone, Copy, PartialEq, Debug)]
pub struct Size {
    pub w: i32,
    pub h: i32,
}

#[derive(Serialize, Deserialize, Clone, Copy, PartialEq, Debug)]
pub struct LiegeSprite {
    pub frame: Frame,
    pub rotated: bool,
    pub trimmed: bool,
    #[serde(rename = "spriteSourceSize")]
    pub sprite_source_size: Frame,
    #[serde(rename = "sourceSize")]
    pub source_size: Size,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct SpriteSheet {
    pub(crate) frames: HashMap<String, LiegeSprite>
}

#[derive(Clone, PartialEq, Debug)]
pub struct LiegeAnimation {
    pub(crate) frames: Vec<LiegeSprite>,
    pub(crate) animation: Animation,
}

#[derive(Clone)]
pub struct LiegeUIAnimation<'a> {
    pub(crate) frames: Vec<LiegeSprite>,
    pub(crate) animation: Animation,
    pub(crate) texture_handle: &'a str,
    pub(crate) position: Vec2,
    pub(crate) state: State,
}

pub(crate) fn read_json_file(file_path: &str) -> Result<String, std::io::Error> {
    fs::read_to_string(file_path)
}

pub(crate) fn deserialize_json(json_str: &str) -> Result<SpriteSheet, serde_json::Error> {
    serde_json::from_str(json_str)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_and_deserialize() {
        let file_content = match read_json_file("resources/characters/rogue/rogue.json") {
            Ok(content) => content,
            Err(e) => panic!("Failed to read file: {}", e),
        };

        let deserialized_content = match  deserialize_json(&file_content) {
            Ok(content) => content,
            Err(e) => panic!("Failed to deserialize: {}", e)
        };
        assert!(!deserialized_content.frames.is_empty(), "Frames hashmap should not be empty");

        let sprite = &deserialized_content.frames["rogue_attack_left_1.png"];
        assert_eq!(sprite.frame.x, 26);
    }
}