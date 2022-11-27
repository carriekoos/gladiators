use bevy::prelude::*;

use crate::gladiator::gladiator_components::*;

#[derive(Debug)]
pub struct AttackEvent {
    pub target: Entity,
    pub attacker: Entity,
    pub attack: Attack,
}

#[derive(Debug)]
pub struct DeathEvent {
    pub victor: Entity,
    pub xp_earned: f32,
    pub slain: Entity,
}
