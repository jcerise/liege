use benimator::{State};
use macroquad::prelude::*;
use crate::LiegeAnimation;

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

#[derive(Clone)]
pub struct AnimatedComponent    {
    pub animated_sprite_label: String,
    pub liege_animation: LiegeAnimation,
    pub animation_state: State,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct EntityKind<'a> {
    pub kind: &'a str
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SelectedComponent {
    pub selected: bool
}