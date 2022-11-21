use bevy::prelude::*;

use crate::{animation::*, ANIMATION_STEP, GLADIATOR_SPEED};

// for some reason I couldn't import this guy.
#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(Timer);

#[derive(Component)]
pub struct Movement {
    pub speed: usize,
}

#[derive(Component)]
pub struct Gladiator;

#[derive(Bundle)]
pub struct GladiatorBundle {
    gladiator: Gladiator,
    movement: Movement,
    animation: Animation,
    animation_timer: AnimationTimer,
}

impl GladiatorBundle {
    pub fn new() -> Self {
        // the default values of the animation will quickly get overwritten
        Self {
            gladiator: Gladiator,
            movement: Movement {
                speed: GLADIATOR_SPEED,
            },
            animation: Animation {
                animation_type: AnimationType::Idle,
                animation_direction: AnimationDirection::Down,
                frame_index: 0,
            },
            animation_timer: AnimationTimer(Timer::from_seconds(
                ANIMATION_STEP,
                TimerMode::Repeating,
            )),
        }
    }
}
