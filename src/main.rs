mod map;

use std::default::Default;
use std::collections::HashMap;
use macroquad::color::{BLACK};
use macroquad::prelude::*;
use crate::map::GameMap;

const TILE_SIZE: f32 = 8.;
const SPRITE_SIZE: f32 = 9.;
const TILE_SCALE: Vec2 = Vec2::new(2.5, 2.5);
const SPRITE_SCALE: Vec2 = Vec2::new(2.5, 2.5);

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
    game_map.generate_noise_map();

    let mut camera = Camera2D {
        target: vec2(screen_width() / 2., screen_height() / 2.),
        zoom: vec2(1. / screen_width() * 2., 1. / screen_height() * 2.),
        ..Default::default()
    };



    loop {
        clear_background(BLACK);

        set_camera(&camera);

        for i in 0..game_map.noise_map.len() {
            let tile_index = game_map.noise_map[i];
            let draw_params = DrawTextureParams{
                source: Option::from(Rect::new((tile_index as f32) * TILE_SIZE, 0., TILE_SIZE, TILE_SIZE)),
                dest_size: Option::from((TILE_SCALE * vec2(TILE_SIZE, TILE_SIZE))),
                ..Default::default()
            };
            let (x, y) = game_map.map_coords(i as i32);
            if is_object_in_view((x as f32) * (TILE_SIZE * TILE_SCALE.x), (y as f32) * (TILE_SIZE * TILE_SCALE.y), &camera) {
                draw_texture_ex(
                    texture_map.get("resources/map/plains.png").unwrap(),
                    (x as f32) * (TILE_SIZE * TILE_SCALE.x),
                    (y as f32) * (TILE_SIZE * TILE_SCALE.y),
                    WHITE,
                    draw_params)
                ;
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

        set_default_camera();

        draw_text(&format!("FPS: {}", get_fps()), 10., 20., 20.0, WHITE);

        next_frame().await;
    }
}

async fn load_resources() -> HashMap<String, Texture2D> {
    let mut texture_assets: HashMap<String, Texture2D> = HashMap::new();
    let texture_paths = vec![
        "resources/map/grass_tiles.png",
        "resources/map/plains.png",
        "resources/characters/rogue/idle/right.png",
    ];

    for path in texture_paths {
        let texture = load_texture(path).await.unwrap();
        texture.set_filter(FilterMode::Nearest);
        texture_assets.insert(path.to_string(), texture);
    }

    // TODO: This seems to not work, find out why --seems to be a known issue with macroquad
    // https://github.com/not-fl3/macroquad/issues/659
    // build_textures_atlas();

    texture_assets
}

fn is_object_in_view(x: f32, y:f32, camera: &Camera2D) -> bool {
    // Calculate the world coordinates of the camera's view boundaries
    let screen_half_width = (screen_width() / 2.0);
    let screen_half_height = (screen_height() / 2.0);

    let left_bound = camera.target.x - screen_half_width - (TILE_SIZE * TILE_SCALE.x);
    let right_bound = camera.target.x + screen_half_width;
    let top_bound = camera.target.y - screen_half_height - (TILE_SIZE * TILE_SCALE.y);
    let bottom_bound = camera.target.y + screen_half_height;

    x > left_bound && x < right_bound && y > top_bound && y < bottom_bound
}