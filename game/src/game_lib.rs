use bevy::prelude::*;

pub mod animation;
pub mod gladiator;
pub mod player;

/// Dumping all of the structs into a lib for a pinch of organization.
/// This should be further broken into plugins, but I don't know how
/// to do that yet ¯\_(ツ)_/¯

pub const CLEAR: Color = Color::rgb(0.1, 0.1, 0.1);
pub const ANIMATION_STEP: f32 = 0.15;
pub const MOVEMENT_STEP: f32 = 1.0 / 60.0; // warning, this is related to PLAYER_SPEED
pub const N_GLADIATORS: i32 = 5;
pub const GLADIATOR_SPEED: usize = 4; // warning, this is related to MOVEMENT_STEP
pub const GLADIATOR_SIZE: f32 = 4.0; // this scales the size of the sprite - lower once there are many
pub const GLADIATOR_SPRITES_PATH: &str = "Puny-Characters/";
// Just starting with a few of the sprites initially
pub const GLADIATOR_SPRITES: [&str; 5] = [
    "Soldier-Blue.png",
    "Soldier-Red.png",
    "Soldier-Yellow.png",
    "Archer-Green.png",
    "Archer-Purple.png",
];

#[derive(Component)]
pub struct Engaged;

#[derive(Component)]
pub struct Unengaged;
