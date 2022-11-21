use bevy::prelude::*;

use crate::gladiator::*;

#[derive(Component)]
pub struct Player;

#[derive(Bundle)]
pub struct PlayerBundle {
    player: Player,
    gladiator: GladiatorBundle,
}

impl PlayerBundle {
    pub fn new() -> Self {
        // the default values of the animation will quickly get overwritten

        Self {
            player: Player,
            gladiator: GladiatorBundle::new(),
        }
    }
}
