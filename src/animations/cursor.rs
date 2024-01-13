use std::collections::HashMap;
use benimator::{Animation, FrameRate, State};
use macroquad::math::Vec2;
use crate::animations::animation::{LiegeAnimation, LiegeUIAnimation};

pub fn load_cursor_animations<'a>() -> HashMap<String, LiegeUIAnimation<'a>> {
    // Load and deserialize our spritesheet information
    let file_content = match crate::animations::animation::read_json_file("resources/ui/cursor/cursor.json") {
        Ok(content) => content,
        Err(e) => panic!("Failed to read file: {}", e),
    };

    let sprite_sheet = match crate::animations::animation::deserialize_json(&file_content) {
        Ok(content) => content,
        Err(e) => panic!("Failed to deserialize: {}", e)
    };

    let mut animations_map: HashMap<String, LiegeUIAnimation> = HashMap::new();

    let idle = LiegeUIAnimation{
        frames: vec![
            sprite_sheet.frames["cursor_default.png"],
        ],
        animation: Animation::from_indices(0..0, FrameRate::from_fps(5.)),
        texture_handle: "resources/ui/cursor/cursor.png",
        position: Vec2::ZERO,
        state: State::new(),
    };

    let click = LiegeUIAnimation{
        frames: vec![
            sprite_sheet.frames["cursor_click_default_1.png"],
            sprite_sheet.frames["cursor_click_default_2.png"],
            sprite_sheet.frames["cursor_click_default_3.png"],
            sprite_sheet.frames["cursor_click_default_4.png"],
        ],
        animation: Animation::from_indices(0..3, FrameRate::from_fps(12.)).once(),
        texture_handle: "resources/ui/cursor/cursor.png",
        position: Vec2::ZERO,
        state: State::new(),
    };

    animations_map.insert("cursor_click".to_string(), click);
    animations_map.insert("cursor_idle".to_string(), idle);

    animations_map
}