use bevy::prelude::*;

use crate::{
    animation::*,
    gladiator::gladiator_components::*,
    gladiator::gladiator_movement::*,
    *, // game_lib
};

#[derive(Bundle)]
pub struct GladiatorBundle {
    // needs Health, Level, Attack, and Defense
    gladiator: Gladiator,
    movement: Movement,
    animation: Animation,
    animation_timer: AnimationTimer,
    attack_timer: AttackTimer,
    health: Health,
    level: Level,
    attack: Attack,
    defense: Defense,
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
                animation_direction: GladiatorDirection::Down,
                frame_index: 0,
            },
            animation_timer: AnimationTimer(Timer::from_seconds(
                ANIMATION_STEP,
                TimerMode::Repeating,
            )),
            attack_timer: AttackTimer(Timer::from_seconds(ATTACK_STEP, TimerMode::Repeating)),
            health: Health { value: 10.0 },
            level: Level { level: 1, xp: 0. },
            attack: Attack { damage: 1.0 },
            defense: Defense { value: 0.1 },
        }
    }
}
