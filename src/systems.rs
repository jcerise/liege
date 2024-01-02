// use legion::system;
// use legion::world::SubWorld;
// use macroquad::math::Vec2;
// use rand::Rng;
// use uuid::Uuid;
// use crate::components::{AnimatedComponent, DrawableComponent, MovementComponent};
// use crate::{AnimationPool, MapInformation, TILE_SCALE, TILE_SIZE};
// use crate::animations::AnimationMap;
//
// #[system(for_each)]
// pub fn apply_random_movement(
//     movement: &mut MovementComponent,
//     drawable: &mut DrawableComponent,
//     animation: &mut AnimatedComponent,
//     #[resource] map_info: &MapInformation,
//     #[resource] mut animation_pool: &mut AnimationPool,
//     #[resource] animation_mapping: &AnimationMap) {
//     let mut rng = rand::thread_rng();
//
//     // Check if this entity has a direction. If not, randomly decide if one should be set, or if it will remain idle
//     if movement.destination == Vec2::ZERO {
//         let chance = 1;
//         let random_number = rng.gen_range(0.0..1.0);
//         if random_number < (chance as f64 / 100.0) {
//             // Create a new destination, scaled from map coordinates to world coordinates
//             let new_dest = Vec2::new(
//                 (rng.gen_range(1..=60) as f32) * (map_info.tile_size * map_info.tile_scale.x),
//                 (rng.gen_range(1..=60) as f32) * (map_info.tile_size * map_info.tile_scale.y)
//             );
//
//             movement.destination = new_dest;
//         } else {
//             // Remain idle, update the animation to idle
//             if animation.animated_sprite_label != "rogue_idle" {
//                 animation.animated_sprite_label = "rogue_idle";
//                 let new_animation = animation_mapping.animations.get("rogue_idle").unwrap().clone();
//                 animation.animated_sprite_index = animation_pool.update_animation(animation.animated_sprite_index, new_animation);
//             }
//         }
//     } else {
//         // This entity has a destination set, so move it towards there
//         // First, get the direction
//         let direction = movement.destination - drawable.position;
//
//         // Check to see if the entity has arrived at (or near) the destination
//         if direction.length() < 1.0 {
//             // Arrived at destination, clear out current destination, and set the idle animation
//             movement.destination = Vec2::ZERO;
//             animation.animated_sprite_label = "rogue_idle";
//             let new_animation = animation_mapping.animations.get("rogue_idle").unwrap().clone();
//             animation.animated_sprite_index = animation_pool.update_animation(animation.animated_sprite_index, new_animation);
//         } else {
//             let normalized_direction = direction.normalize();
//             // Move towards the destination, setting an appropriate animation based on the direction of movement
//             drawable.position += normalized_direction * movement.speed;
//
//             // Calculate the correct animation to apply during movement
//             let mut new_animation = "rogue_idle";
//             if normalized_direction[0] > 0. && normalized_direction[1] > 0. {
//                 // Heading right and down
//                 new_animation = "rogue_walk_left";
//             } else if normalized_direction[0] < 0. && normalized_direction[1] > 0. {
//                 // Heading left and down
//                 new_animation = "rogue_walk_right";
//             } else if normalized_direction[0] > 0. && normalized_direction[1] < 0. {
//                 // Heading right and up
//                 new_animation = "rogue_walk_up_left";
//             } else if normalized_direction[0] < 0. && normalized_direction[1] < 0. {
//                 // Heading left and up
//                 new_animation = "rogue_walk_up_right";
//             }
//
//             if animation.animated_sprite_label != new_animation {
//                 animation.animated_sprite_label = new_animation;
//                 let new_animation_sprite = animation_mapping.animations.get(new_animation).unwrap().clone();
//                 animation.animated_sprite_index = animation_pool.update_animation(animation.animated_sprite_index, new_animation_sprite);
//             }
//         }
//     }
// }