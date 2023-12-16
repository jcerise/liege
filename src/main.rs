mod map;

use std::default::Default;
use std::collections::HashMap;
use macroquad::color::{BLACK};
use macroquad::prelude::*;
use crate::map::GameMap;

fn conf() -> Conf {
    Conf {
        window_title: "Liege".to_string(),
        window_width: 1024,
        window_height: 768,
        ..Default::default()
    }
}

#[macroquad::main(conf)]
async fn main() {
    let mut texture_map = load_resources().await;
    let mut game_map = GameMap::new(200, 200);
    game_map.generate_simple_map();

    let mut camera = Camera2D {
        zoom: vec2(1. / screen_width() * 2., 1. / screen_height() * 2.),
        target: vec2(screen_width() / 2., screen_height() / 2.),
        ..Default::default()
    };

    loop {
        clear_background(BLACK);

        set_camera(&camera);

        for i in 0..game_map.tiles.len() {
            let tile_index = game_map.tiles[i];
            let draw_params = DrawTextureParams{
                source: Option::from(Rect::new((tile_index * 24) as f32, 0., 24., 24.)),
                ..Default::default()
            };
            let (x, y) = game_map.map_coords(i as i32);
            if is_object_in_view((x * 24) as f32, (y * 24) as f32, &camera) {
                draw_texture_ex(texture_map.get("resources/map/grass_tiles.png").unwrap(), (x * 24) as f32, (y * 24) as f32, WHITE, draw_params);
            }
        }

        if is_key_down(KeyCode::Up) {
            camera.target.y -= 4.0;
        }
        if is_key_down(KeyCode::Down) {
            camera.target.y += 4.0;
        }
        if is_key_down(KeyCode::Left) {
            camera.target.x -= 4.0;
        }
        if is_key_down(KeyCode::Right) {
            camera.target.x += 4.0;
        }

        let text_pos_x = camera.target.x - screen_width() / 2.0;
        let text_pos_y = camera.target.y - screen_height() / 2.0;
        draw_text(&format!("FPS: {}", get_fps()), text_pos_x + 20., text_pos_y + 20., 20.0, WHITE);

        set_default_camera();

        next_frame().await;
    }
}

async fn load_resources() -> HashMap<String, Texture2D> {
    let mut texture_assets: HashMap<String, Texture2D> = HashMap::new();
    let texture_paths = vec![
        "resources/map/grass_tiles.png"
    ];

    for path in texture_paths {
        texture_assets.insert(path.to_string(), load_texture(path).await.unwrap());
    }

    // TODO: This seems to not work, find out why
    // build_textures_atlas();

    texture_assets
}

fn is_object_in_view(x: f32, y:f32, camera: &Camera2D) -> bool {
    // Calculate the world coordinates of the camera's view boundaries
    let screen_half_width = screen_width() / 2.0;
    let screen_half_height = screen_height() / 2.0;

    let left_bound = camera.target.x - screen_half_width - 24.;
    let right_bound = camera.target.x + screen_half_width;
    let top_bound = camera.target.y - screen_half_height - 24.;
    let bottom_bound = camera.target.y + screen_half_height;

    x > left_bound && x < right_bound && y > top_bound && y < bottom_bound
}