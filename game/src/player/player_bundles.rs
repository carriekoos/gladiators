use bevy::prelude::*;

use crate::gladiator::{gladiator::*, gladiator_bundles::*};
use crate::player::player_components::*;

#[derive(Bundle)]
pub struct PlayerBundle {
    player: Player,
    gladiator: GladiatorBundle,
}

impl PlayerBundle {
    pub fn new() -> Self {
        Self {
            player: Player,
            gladiator: GladiatorBundle::new(Class::Mage),
        }
    }
}
