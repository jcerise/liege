mod map;
mod animations;
mod components;
mod systems;

use std::default::Default;
use std::collections::HashMap;
use legion::{IntoQuery, Read, Resources, Schedule, World, Write};
use macroquad::color::{BLACK};
use macroquad::experimental::animation::{AnimatedSprite, Animation};
use macroquad::prelude::*;
use uuid::Uuid;
use crate::animations::{AnimationMap, AnimationPool, load_animations};
use crate::components::{AnimatedComponent, AnimationState, DrawableComponent, MovementComponent};
use crate::map::GameMap;
use crate::systems::apply_random_movement_system;

const TILE_SIZE: f32 = 8.;
const SPRITE_SIZE: f32 = 9.;
const TILE_SCALE: Vec2 = Vec2::new(2.5, 2.5);
const SPRITE_SCALE: Vec2 = Vec2::new(2.5, 2.5);
const MAP_WIDTH: i32 = 200;
const MAP_HEIGHT: i32 = 200;

struct RenderData<'a> {
    position: Vec2,
    texture_handle: &'a str,
    animation_index: Uuid,
}

pub struct MapInformation {
    width: i32,
    height: i32,
    tile_size: f32,
    tile_scale: Vec2,
}

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
    let mut game_map = GameMap::new(MAP_WIDTH, MAP_HEIGHT);
    game_map.generate_noise_map();

    let mut animation_map = load_animations();

    let mut camera = Camera2D {
        target: vec2(screen_width() / 2., screen_height() / 2.),
        zoom: vec2(1. / screen_width() * 2., 1. / screen_height() * 2.),
        ..Default::default()
    };

    let mut world = World::default();
    let mut resources = Resources::default();

    resources.insert(MapInformation{width: MAP_WIDTH, height: MAP_HEIGHT, tile_size: TILE_SIZE, tile_scale: TILE_SCALE});
    resources.insert(AnimationPool{pool: HashMap::new()});
    resources.insert(AnimationMap{animations: animation_map});

    for _ in 0..1000 {
        // Create a single rogue entity
        let id = Uuid::new_v4();
        if let Some(mut animation_mapping) = resources.get_mut::<AnimationMap>() {
            let animation = animation_mapping.animations.get("rogue_idle").unwrap().clone();
            world.push(
                (
                    DrawableComponent {
                        position: Vec2::new(100., 100.),
                        texture_handle: "resources/characters/rogue/rogue.png"
                    },
                    AnimatedComponent {
                        animated_sprite_label: "rogue_idle",
                        animated_sprite_index: id
                    },
                    MovementComponent{ destination: Vec2::ZERO, speed: 0.5}
                )
            );
            // Push the animation into the animation pool
            if let Some(mut animation_pool) = resources.get_mut::<AnimationPool>() {
                animation_pool.pool.insert(id, animation);
            }
        }
    }

    let mut schedule = Schedule::builder()
        .add_system(apply_random_movement_system())
        .build();

    loop {
        clear_background(BLACK);

        // Execute all systems
        schedule.execute(&mut world, &mut resources);

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

        let mut render_data = Vec::new();
        let mut query = <(Read<DrawableComponent>, Read<AnimatedComponent>)>::query();
        for (drawable, animated) in query.iter(&world) {
            render_data.push(RenderData {
                position: drawable.position,
                texture_handle: drawable.texture_handle,
                animation_index: animated.animated_sprite_index,
            });
        }

        for data in render_data {
            if let Some(mut animation_pool) = resources.get_mut::<AnimationPool>() {
                let current_anim = animation_pool.pool.get(&data.animation_index).unwrap();
                let draw_params = DrawTextureParams{
                    source: Some(current_anim.frame().source_rect),
                    dest_size: Option::from((SPRITE_SCALE * vec2(SPRITE_SIZE, SPRITE_SIZE))),
                    ..Default::default()
                };

                draw_texture_ex(texture_map.get(data.texture_handle).unwrap(), data.position.x, data.position.y, WHITE, draw_params);
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

        // Debug information, printed in screen space
        let entity_count = world.len();
        draw_text(&format!("FPS: {}", get_fps()), 10., 20., 20., WHITE);
        draw_text(&format!("Entity Count: {}", entity_count), 10., 30., 20., WHITE);
        if let Some(mut animation_pool) = resources.get_mut::<AnimationPool>() {
            let animations_count = animation_pool.pool.len();
            draw_text(&format!("Animations Count: {}", animations_count), 10., 40., 20., WHITE);
        }

        // Update all animations across all entities
        if let Some(mut animation_pool) = resources.get_mut::<AnimationPool>() {
            for (_, animation) in animation_pool.pool.iter_mut() {
                animation.update();
            }
        }

        next_frame().await;
    }
}

async fn load_resources() -> HashMap<String, Texture2D> {
    let mut texture_assets: HashMap<String, Texture2D> = HashMap::new();
    let texture_paths = vec![
        "resources/map/grass_tiles.png",
        "resources/map/plains.png",
        "resources/characters/rogue/rogue.png"
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