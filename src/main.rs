mod map;
mod components;
mod systems;
mod animations;

use crate::systems::{apply_random_death_system, apply_random_movement_system};
use std::default::Default;
use std::collections::HashMap;
use std::time::Duration;
use benimator::State;
use legion::{Entity, IntoQuery, Read, Resources, Schedule, World, Write};
use macroquad::color::{BLACK};
use macroquad::prelude::*;
use rand::Rng;
use crate::animations::animation::{AnimationMap, AnimationStates, LiegeAnimation, LiegeSprite, LiegeUIAnimation, UIAnimationMap};
use crate::animations::cursor::load_cursor_animations;
use crate::animations::goblin::{load_goblin_animations};
use crate::animations::rogue::{load_rogue_animations};
use crate::components::{AnimatedComponent, EntityKind, DrawableComponent, MovementComponent, SelectedComponent};
use crate::map::GameMap;
extern crate rand;

const TILE_SIZE: f32 = 8.;
const SPRITE_SIZE: f32 = 9.;
const TILE_SCALE: Vec2 = Vec2::new(2.5, 2.5);
const SPRITE_SCALE: Vec2 = Vec2::new(2.5, 2.5);
const MAP_WIDTH: i32 = 200;
const MAP_HEIGHT: i32 = 200;

struct RenderData<'a> {
    position: Vec2,
    texture_handle: &'a str,
    animation: LiegeAnimation,
    state: State,
    selected: bool,
}

pub struct MapInformation {
    width: i32,
    height: i32,
    tile_size: f32,
    tile_scale: Vec2,
}

pub enum CreatureType {
    Rogue,
    Goblin,
}

impl CreatureType {
    pub fn to_str(&self) -> &str {
        match self {
            CreatureType::Rogue => "rogue",
            CreatureType::Goblin => "goblin",
        }
    }
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
    let mut show_debug = false;
    let mut paused = false;

    // Hide the cursor, so we can draw our own custom cursor, and create an animation to use for the cursor
    show_mouse(false);


    let mut rng = rand::thread_rng();

    let mut texture_map = load_resources().await;
    let mut game_map = GameMap::new(MAP_WIDTH, MAP_HEIGHT);
    game_map.generate_noise_map();

    let mut animation_map = load_rogue_animations();
    animation_map.extend(load_goblin_animations());

    let mut ui_animation_map = load_cursor_animations();

    let mut camera = Camera2D {
        target: vec2(screen_width() / 2., screen_height() / 2.),
        zoom: vec2(1. / screen_width() * 2., 1. / screen_height() * 2.),
        ..Default::default()
    };

    let mut world = World::default();
    let mut resources = Resources::default();

    resources.insert(MapInformation{width: MAP_WIDTH, height: MAP_HEIGHT, tile_size: TILE_SIZE, tile_scale: TILE_SCALE});
    resources.insert(AnimationMap{animations: animation_map});
    resources.insert(UIAnimationMap{animations: ui_animation_map});

    for _ in 0..5 {
        // Create a single rogue entity
        if let Some(mut animation_mapping) = resources.get_mut::<AnimationMap>() {
            let animation = animation_mapping.animations.get("rogue_idle_right").unwrap().clone();
            world.push(
                (
                    DrawableComponent {
                        position: Vec2::new(100., 100.),
                        texture_handle: "resources/characters/rogue/rogue.png"
                    },
                    AnimatedComponent {
                        animated_sprite_label: "rogue_idle_right".to_string(),
                        liege_animation: animation,
                        animation_state: State::new()
                    },
                    MovementComponent{ destination: Vec2::ZERO, speed: 0.5},
                    EntityKind { kind: CreatureType::Rogue.to_str() },
                    SelectedComponent { selected: false }
                )
            );
        }
    }

    for _ in 0..5 {
        // Create a single goblin entity
        if let Some(mut animation_mapping) = resources.get_mut::<AnimationMap>() {
            let animation = animation_mapping.animations.get("goblin_idle_right").unwrap().clone();
            world.push(
                (
                    DrawableComponent {
                        position: Vec2::new(100., 100.),
                        texture_handle: "resources/characters/goblin/goblin.png"
                    },
                    AnimatedComponent {
                        animated_sprite_label: "goblin_idle_right".to_string(),
                        liege_animation: animation,
                        animation_state: State::new()
                    },
                    MovementComponent{ destination: Vec2::ZERO, speed: 0.5},
                    EntityKind { kind: CreatureType::Goblin.to_str() },
                    SelectedComponent { selected: false }
                )
            );
        }
    }

    let mut schedule = Schedule::builder()
        .add_system(apply_random_movement_system())
        //.add_system(apply_random_death_system())
        .build();

    // Build a vector of current UI animations that will be run every frame
    let mut current_ui_animations: Vec<LiegeUIAnimation> = Vec::new();

    loop {
        clear_background(BLACK);

        // Execute all systems
        if !paused {
            schedule.execute(&mut world, &mut resources);
        }

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
        let mut query = <(Read<DrawableComponent>, Read<AnimatedComponent>, Read<SelectedComponent>)>::query();
        for (drawable, animated, selected) in query.iter(&world) {
            render_data.push(RenderData {
                position: drawable.position,
                texture_handle: drawable.texture_handle,
                animation: animated.liege_animation.clone(),
                state: animated.animation_state.clone(),
                selected: selected.selected,
            });
        }

        for data in render_data {
            if is_sprite_in_view(data.position.x + (SPRITE_SIZE * SPRITE_SCALE.x), data.position.y + (SPRITE_SIZE * SPRITE_SCALE.y), &camera) {
                let frame: LiegeSprite = data.animation.frames[data.state.frame_index()].clone();
                let source_rect = Rect::new(frame.frame.x as f32, frame.frame.y as f32, frame.frame.w as f32, frame.frame.h as f32);
                let draw_params = DrawTextureParams{
                    source: Some(source_rect),
                    dest_size: Option::from((SPRITE_SCALE * vec2(frame.source_size.w as f32, frame.source_size.h as f32))),
                    ..Default::default()
                };

                draw_texture_ex(texture_map.get(data.texture_handle).unwrap(), data.position.x, data.position.y, WHITE, draw_params);

                if data.selected {
                    // This entity is currently selected by the player, draw a selection rectangle around the entity sprite
                    let scaled_size = SPRITE_SCALE * vec2(frame.frame.w as f32, frame.frame.h as f32);
                    draw_rectangle_lines(data.position.x, data.position.y, scaled_size.x, scaled_size.y, 2.0, RED);
                }
            }
        }

        // Draw our custom mouse cursor
        let (mouse_x, mouse_y) = mouse_position();
        if let Some(mut animation_mapping) = resources.get_mut::<UIAnimationMap>() {
            if let Some(cursor_animation) = animation_mapping.animations.get("cursor_idle") {
                let frame: LiegeSprite = cursor_animation.frames[0].clone();
                let source_rect = Rect::new(frame.frame.x as f32, frame.frame.y as f32, frame.frame.w as f32, frame.frame.h as f32);
                let draw_params = DrawTextureParams{
                    source: Some(source_rect),
                    dest_size: Option::from((SPRITE_SCALE * vec2(frame.source_size.w as f32, frame.source_size.h as f32))),
                    ..Default::default()
                };

                draw_texture_ex(texture_map.get("resources/ui/cursor/cursor.png").unwrap(), mouse_x, mouse_y, WHITE, draw_params);
            }

        }

        // Check for mouse clicks, capture the position
        if is_mouse_button_pressed(MouseButton::Left) {
            let click_position = camera.screen_to_world(Vec2::from(mouse_position()));

            // Before we do anything else, kick off a new UI animation for the click
            if let Some(mut ui_animation_mapping) = resources.get_mut::<UIAnimationMap>() {
                let mut click_anim = ui_animation_mapping.animations.get("cursor_click").unwrap().clone();
                click_anim.position = click_position - 8.;
                current_ui_animations.push(click_anim);
            }

            let mut query = <(Read<DrawableComponent>, Read<AnimatedComponent>, Read<EntityKind>, Write<SelectedComponent>)>::query();
            for (drawable, animated, kind, selected) in query.iter_mut(&mut world) {
                // Check all entities for the coordinates of the mouse click
                let frame = animated.liege_animation.frames[animated.animation_state.frame_index()];
                let scaled_size = SPRITE_SCALE * vec2(frame.frame.w as f32, frame.frame.h as f32);
                if click_position.x >= drawable.position.x && click_position.x <= drawable.position.x + scaled_size.x &&
                    click_position.y >= drawable.position.y && click_position.y <= drawable.position.y + scaled_size.y {
                    selected.selected = true;
                } else {
                    // This entity was not selected, so mark it as not selected. This will also clear current selection
                    // if empty space is clicked
                    selected.selected = false;
                }
            }
        } else if is_mouse_button_pressed(MouseButton::Right) {
            // Spawn a new random entity (Rogue or Goblin, currently)
            let random_choice = rng.gen_range(0..2);
            let click_position = camera.screen_to_world(Vec2::from(mouse_position()));
            if random_choice == 0 {
                // Create a Rogue
                if let Some(mut animation_mapping) = resources.get_mut::<AnimationMap>() {
                    let animation = animation_mapping.animations.get("rogue_idle_right").unwrap().clone();
                    world.push(
                        (
                            DrawableComponent {
                                position: Vec2::new(click_position.x, click_position.y),
                                texture_handle: "resources/characters/rogue/rogue.png"
                            },
                            AnimatedComponent {
                                animated_sprite_label: "rogue_idle_right".to_string(),
                                liege_animation: animation,
                                animation_state: State::new()
                            },
                            MovementComponent{ destination: Vec2::ZERO, speed: 0.5},
                            EntityKind { kind: CreatureType::Rogue.to_str() },
                            SelectedComponent { selected: false }
                        )
                    );
                }
            } else {
                // Create a Goblin
                if let Some(mut animation_mapping) = resources.get_mut::<AnimationMap>() {
                    let animation = animation_mapping.animations.get("goblin_idle_right").unwrap().clone();
                    world.push(
                        (
                            DrawableComponent {
                                position: Vec2::new(click_position.x, click_position.y),
                                texture_handle: "resources/characters/goblin/goblin.png"
                            },
                            AnimatedComponent {
                                animated_sprite_label: "goblin_idle_right".to_string(),
                                liege_animation: animation,
                                animation_state: State::new()
                            },
                            MovementComponent{ destination: Vec2::ZERO, speed: 0.5},
                            EntityKind { kind: CreatureType::Goblin.to_str() },
                            SelectedComponent { selected: false }
                        )
                    );
                }
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
        if is_key_pressed(KeyCode::Space) {
            paused = !paused;
        }
        if is_key_down(KeyCode::LeftShift) && is_key_down(KeyCode::D) {
            show_debug = !show_debug;
        }

        set_default_camera();

        if show_debug {
            // Debug information, printed in screen space
            let entity_count = world.len();
            draw_text(&format!("FPS: {}", get_fps()), 10., 20., 20., WHITE);
            draw_text(&format!("Entity Count: {}", entity_count), 10., 31., 20., WHITE);
            draw_text(&format!("Current Mouse Position (Screen): {}", Vec2::from(mouse_position())), 10., 42., 20., WHITE);
            draw_text(&format!("Current Mouse Position (World): {}", camera.screen_to_world(Vec2::from(mouse_position()))), 10., 53., 20., WHITE);

            let mut query = <(Entity, Read<EntityKind>, Read<SelectedComponent>)>::query();
            for (entity, kind, selected) in query.iter(&world) {
                if selected.selected {
                    draw_text(&format!("Selected Entity: {:?}, {}", entity, kind.kind), 10., 64., 20., WHITE);
                }
            }
        }

        let nanos = (get_frame_time() * 1_000_000_000.0) as u64;
        if !paused {
            let mut query = <(Write<AnimatedComponent>)>::query();
            for mut animated in query.iter_mut(&mut world) {
                // Weird: We have to convert to nanoseconds, or we lose precision in the Duration object, and our animations
                // never update.
                animated.animation_state.update(&animated.liege_animation.animation, Duration::from_nanos(nanos));
            }
        }

        // Run any UI animtations
        for mut animation in current_ui_animations.iter_mut() {
            let frame: LiegeSprite = animation.frames[animation.state.frame_index()].clone();
            let source_rect = Rect::new(frame.frame.x as f32, frame.frame.y as f32, frame.frame.w as f32, frame.frame.h as f32);
            let draw_params = DrawTextureParams{
                source: Some(source_rect),
                dest_size: Option::from((SPRITE_SCALE * vec2(frame.source_size.w as f32, frame.source_size.h as f32))),
                ..Default::default()
            };

            draw_texture_ex(texture_map.get(animation.texture_handle).unwrap(), animation.position.x, animation.position.y, WHITE, draw_params);

            animation.state.update(&animation.animation, Duration::from_nanos(nanos));
        }
        // Clear out any UI animations that have finished running
        current_ui_animations.retain(|x| !x.state.is_ended());

        next_frame().await;
    }
}

async fn load_resources() -> HashMap<String, Texture2D> {
    let mut texture_assets: HashMap<String, Texture2D> = HashMap::new();
    let texture_paths = vec![
        "resources/map/grass_tiles.png",
        "resources/map/plains.png",
        "resources/characters/rogue/rogue.png",
        "resources/characters/goblin/goblin.png",
        "resources/ui/cursor/cursor.png",
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

fn is_sprite_in_view(x: f32, y:f32, camera: &Camera2D) -> bool {
    // Calculate the world coordinates of the camera's view boundaries
    let screen_half_width = (screen_width() / 2.0);
    let screen_half_height = (screen_height() / 2.0);

    let left_bound = camera.target.x - screen_half_width - (SPRITE_SIZE * SPRITE_SCALE.x);
    let right_bound = camera.target.x + screen_half_width;
    let top_bound = camera.target.y - screen_half_height - (SPRITE_SIZE * SPRITE_SCALE.y);
    let bottom_bound = camera.target.y + screen_half_height;

    x > left_bound && x < right_bound && y > top_bound && y < bottom_bound
}