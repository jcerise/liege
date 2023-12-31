use macroquad::experimental::animation::AnimatedSprite;
use macroquad::prelude::*;
use uuid::Uuid;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AnimationState {
    Idle,
    WalkRight,
    WalkLeft,
    WalkUpRight,
    WalkUpLeft,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DrawableComponent<'a> {
    pub(crate) position: Vec2,
    pub texture_handle: &'a str,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MovementComponent {
    pub destination: Vec2,
    pub speed: f32,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct AnimatedComponent<'a> {
    pub animated_sprite_label: &'a str,
    pub animated_sprite_index: Uuid
}