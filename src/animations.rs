use std::collections::HashMap;
use macroquad::experimental::animation::{AnimatedSprite, Animation};

struct SimpleAnimation {
    width: f32,
    height: f32,
    animation: Animation,
    playing: bool
}

pub fn load_animations() -> HashMap<String, AnimatedSprite> {
    let mut animations_map: HashMap<String, AnimatedSprite> = HashMap::new();

    let rogue_idle = AnimatedSprite::new(
        9,
        9,
        &[
            Animation {
                name: "idle".to_string(),
                row: 0,
                frames: 16,
                fps: 6
            }
        ],
        true,
    );

    let rogue_walk_right = AnimatedSprite::new(
        9,
        9,
        &[
            Animation {
                name: "rogue_walk_right".to_string(),
                row: 1,
                frames: 4,
                fps: 4
            }
        ],
        true
    );

    let rogue_walk_left = AnimatedSprite::new(
        9,
        9,
        &[
            Animation {
                name: "rogue_walk_left".to_string(),
                row: 2,
                frames: 4,
                fps: 4
            }
        ],
        true
    );
    let rogue_walk_up_right = AnimatedSprite::new(
        9,
        9,
        &[
            Animation {
                name: "rogue_walk_up_right".to_string(),
                row: 3,
                frames: 4,
                fps: 4
            }
        ],
        true
    );
    let rogue_walk_up_left = AnimatedSprite::new(
        9,
        9,
        &[
            Animation {
                name: "rogue_walk_up_left".to_string(),
                row: 4,
                frames: 4,
                fps: 4
            }
        ],
        true
    );

    animations_map.insert("rogue_idle".to_string(), rogue_idle);
    animations_map.insert("rogue_walk_right".to_string(), rogue_walk_right);
    animations_map.insert("rogue_walk_left".to_string(), rogue_walk_left);
    animations_map.insert("rogue_walk_up_right".to_string(), rogue_walk_up_right);
    animations_map.insert("rogue_walk_up_left".to_string(), rogue_walk_up_left);

    animations_map
}

