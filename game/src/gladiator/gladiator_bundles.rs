use bevy::prelude::*;

use crate::{
    animation::*,
    gladiator::gladiator::*,
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
    class: GladiatorClass,
}

impl GladiatorBundle {
    pub fn new(gladiator_class: Class) -> Self {
        // determine class-specific values
        // TODO replace with constants in game_lib or maybe have modifier constants that
        //  multiply by a base value here.
        // This could consist of a few helper functions to clean it up and tie in the balancing
        //  hyper parameter for all of the math to generate these values.
        let (health, attack_speed, damage, defense, speed, class_xp_modifier) =
            match gladiator_class {
                Class::Archer => (
                    10.0,
                    ATTACK_STEP * 1.1,
                    2.0,
                    0.2,
                    GLADIATOR_BASE_SPEED * 1.2,
                    1.0,
                ),
                Class::Mage => (
                    8.0,
                    ATTACK_STEP * 1.2,
                    1.0,
                    0.1,
                    GLADIATOR_BASE_SPEED * 0.8,
                    1.1,
                ),
                Class::Fighter => (
                    15.0,
                    ATTACK_STEP * 1.0,
                    1.0,
                    0.5,
                    GLADIATOR_BASE_SPEED * 1.0,
                    0.9,
                ),
            };

        Self {
            gladiator: Gladiator,
            movement: Movement { speed },
            animation: Animation {
                animation_type: AnimationType::Idle,
                animation_direction: GladiatorDirection::Down,
                frame_index: 0,
            },
            animation_timer: AnimationTimer(Timer::from_seconds(
                ANIMATION_STEP,
                TimerMode::Repeating,
            )),
            attack_timer: AttackTimer(Timer::from_seconds(attack_speed, TimerMode::Repeating)),
            health: Health { value: health },
            level: Level {
                level: 1,
                xp: 0.,
                class_xp_modifier,
            },
            attack: Attack { damage },
            defense: Defense { value: defense },
            class: GladiatorClass {
                class: gladiator_class,
            },
        }
    }
}
