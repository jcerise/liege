use benimator::State;
use legion::{Entity, system};
use legion::systems::CommandBuffer;
use macroquad::math::Vec2;
use rand::Rng;
use crate::components::{AnimatedComponent, CreatureType, DrawableComponent, MovementComponent};
use crate::{MapInformation};
use crate::animations::animation::AnimationMap;
use crate::animations::goblin::GoblinAnimationStates;
use crate::animations::rogue::RogueAnimationStates;

#[system(for_each)]
pub fn apply_random_rogue_death(
    entity: &Entity,
    animation: &mut AnimatedComponent,
    creature_type: &CreatureType,
    cmd: &mut CommandBuffer,
    #[resource] animation_mapping: &AnimationMap, ) {
    if creature_type.creature_type == "rogue" && animation.animated_sprite_label != RogueAnimationStates::RogueDie.to_string() {
        // Randomly kill rogues
        let mut rng = rand::thread_rng();
        let chance = 0.005;
        let random_number = rng.gen_range(0.0..1.0);
        if random_number < (chance as f64 / 100.0) {
            // Change the animation to the death animation
            let new_animation = RogueAnimationStates::RogueDie.to_string();
            animation.animated_sprite_label = new_animation;
            animation.liege_animation = animation_mapping.animations.get(new_animation).unwrap().clone();
            animation.animation_state = State::new();
        }
    } else if creature_type.creature_type == "rogue" && animation.animated_sprite_label == RogueAnimationStates::RogueDie.to_string() {
        // If the rogue is already dying, check if the animation has ended, and remove the entity when that occurs (after the last frame)
        if animation.animation_state.is_ended() {
            cmd.remove(*entity);
        }
    }
}

#[system(for_each)]
pub fn apply_random_movement_rogue(
    movement: &mut MovementComponent,
    drawable: &mut DrawableComponent,
    animation: &mut AnimatedComponent,
    creature_type: &mut CreatureType,
    #[resource] map_info: &MapInformation,
    #[resource] animation_mapping: &AnimationMap) {
    if creature_type.creature_type == "rogue" && animation.animated_sprite_label != RogueAnimationStates::RogueDie.to_string() {
        let mut rng = rand::thread_rng();

        // Check if this entity has a direction. If not, randomly decide if one should be set, or if it will remain idle
        if movement.destination == Vec2::ZERO {
            let chance = 0.5;
            let random_number = rng.gen_range(0.0..1.0);
            if random_number < (chance as f64 / 100.0) {
                // Create a new destination, scaled from map coordinates to world coordinates
                let new_dest = Vec2::new(
                    (rng.gen_range(1..=60) as f32) * (map_info.tile_size * map_info.tile_scale.x),
                    (rng.gen_range(1..=60) as f32) * (map_info.tile_size * map_info.tile_scale.y)
                );

                movement.destination = new_dest;
            } else {
                // Remain idle, update the animation to idle
                if animation.animated_sprite_label != RogueAnimationStates::RogueIdleRight.to_string() &&
                    animation.animated_sprite_label != RogueAnimationStates::RogueIdleLeft.to_string()
                {
                    let new_animation = RogueAnimationStates::RogueIdleRight.to_string();
                    animation.animated_sprite_label = new_animation;
                    animation.liege_animation = animation_mapping.animations.get(new_animation).unwrap().clone();
                    animation.animation_state = State::new();
                }
            }
        } else {
            // This entity has a destination set, so move it towards there
            // First, get the direction
            let direction = movement.destination - drawable.position;

            // Check to see if the entity has arrived at (or near) the destination
            if direction.length() < 1.0 {
                // Arrived at destination, clear out current destination, and set the idle animation
                movement.destination = Vec2::ZERO;

                let new_animation = RogueAnimationStates::RogueIdleRight.to_string();
                animation.animated_sprite_label = new_animation;
                animation.liege_animation = animation_mapping.animations.get(new_animation).unwrap().clone();
                animation.animation_state = State::new();
            } else {
                let normalized_direction = direction.normalize();
                // Move towards the destination, setting an appropriate animation based on the direction of movement
                drawable.position += normalized_direction * movement.speed;

                // Calculate the correct animation to apply during movement
                let mut new_animation = RogueAnimationStates::RogueIdleRight.to_string();
                if normalized_direction[0] > 0. && normalized_direction[1] > 0. {
                    // Heading right and down
                    new_animation = RogueAnimationStates::RogueWalkRight.to_string();
                } else if normalized_direction[0] < 0. && normalized_direction[1] > 0. {
                    // Heading left and down
                    new_animation = RogueAnimationStates::RogueWalkLeft.to_string();
                } else if normalized_direction[0] > 0. && normalized_direction[1] < 0. {
                    // Heading right and up
                    new_animation = RogueAnimationStates::RogueWalkUpRight.to_string();;
                } else if normalized_direction[0] < 0. && normalized_direction[1] < 0. {
                    // Heading left and up
                    new_animation = RogueAnimationStates::RogueWalkUpLeft.to_string();;
                }

                if animation.animated_sprite_label != new_animation {
                    animation.animated_sprite_label = new_animation;
                    animation.liege_animation = animation_mapping.animations.get(new_animation).unwrap().clone();
                    animation.animation_state = State::new();
                }
            }
        }
    }
}

#[system(for_each)]
pub fn apply_random_movement_goblin(
    movement: &mut MovementComponent,
    drawable: &mut DrawableComponent,
    animation: &mut AnimatedComponent,
    creature_type: &mut CreatureType,
    #[resource] map_info: &MapInformation,
    #[resource] animation_mapping: &AnimationMap) {
    if creature_type.creature_type == "goblin" {
        let mut rng = rand::thread_rng();

        // Check if this entity has a direction. If not, randomly decide if one should be set, or if it will remain idle
        if movement.destination == Vec2::ZERO {
            let chance = 0.5;
            let random_number = rng.gen_range(0.0..1.0);
            if random_number < (chance as f64 / 100.0) {
                // Create a new destination, scaled from map coordinates to world coordinates
                let new_dest = Vec2::new(
                    (rng.gen_range(1..=60) as f32) * (map_info.tile_size * map_info.tile_scale.x),
                    (rng.gen_range(1..=60) as f32) * (map_info.tile_size * map_info.tile_scale.y)
                );

                movement.destination = new_dest;
            } else {
                // Remain idle, update the animation to idle
                if animation.animated_sprite_label != GoblinAnimationStates::GoblinIdleRight.to_string() &&
                    animation.animated_sprite_label != GoblinAnimationStates::GoblinIdleLeft.to_string()
                {
                    let new_animation = GoblinAnimationStates::GoblinIdleRight.to_string();
                    animation.animated_sprite_label = new_animation;
                    animation.liege_animation = animation_mapping.animations.get(new_animation).unwrap().clone();
                    animation.animation_state = State::new();
                }
            }
        } else {
            // This entity has a destination set, so move it towards there
            // First, get the direction
            let direction = movement.destination - drawable.position;

            // Check to see if the entity has arrived at (or near) the destination
            if direction.length() < 1.0 {
                // Arrived at destination, clear out current destination, and set the idle animation
                movement.destination = Vec2::ZERO;

                let new_animation = GoblinAnimationStates::GoblinIdleRight.to_string();
                animation.animated_sprite_label = new_animation;
                animation.liege_animation = animation_mapping.animations.get(new_animation).unwrap().clone();
                animation.animation_state = State::new();
            } else {
                let normalized_direction = direction.normalize();
                // Move towards the destination, setting an appropriate animation based on the direction of movement
                drawable.position += normalized_direction * movement.speed;

                // Calculate the correct animation to apply during movement
                let mut new_animation = GoblinAnimationStates::GoblinIdleRight.to_string();
                if normalized_direction[0] > 0. && normalized_direction[1] > 0. {
                    // Heading right and down
                    new_animation = GoblinAnimationStates::GoblinWalkRight.to_string();
                } else if normalized_direction[0] < 0. && normalized_direction[1] > 0. {
                    // Heading left and down
                    new_animation = GoblinAnimationStates::GoblinWalkLeft.to_string();
                } else if normalized_direction[0] > 0. && normalized_direction[1] < 0. {
                    // Heading right and up
                    new_animation = GoblinAnimationStates::GoblinWalkUpRight.to_string();;
                } else if normalized_direction[0] < 0. && normalized_direction[1] < 0. {
                    // Heading left and up
                    new_animation = GoblinAnimationStates::GoblinWalkUpLeft.to_string();;
                }

                if animation.animated_sprite_label != new_animation {
                    animation.animated_sprite_label = new_animation;
                    animation.liege_animation = animation_mapping.animations.get(new_animation).unwrap().clone();
                    animation.animation_state = State::new();
                }
            }
        }
    }
}