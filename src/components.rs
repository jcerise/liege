use macroquad::experimental::animation::AnimatedSprite;
use macroquad::prelude::*;

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
    pub velocity: Vec2,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct AnimatedComponent<'a> {
    pub animated_sprite_index: &'a str
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct AnimationStateComponent {
    pub state: AnimationState
}