use bevy::prelude::*;

pub mod animation;
pub mod gladiator;
pub mod grid;
pub mod player;

/// Dumping all of the structs into a lib for a pinch of organization.
/// This should be further broken into plugins, but I don't know how
/// to do that yet ¯\_(ツ)_/¯

pub const CLEAR: Color = Color::rgb(0.1, 0.1, 0.1);

/// Grid constants
pub const HORIZONTAL_DIVISIONS: f32 = 20.; // vertical divisions calculated by this * (window) height/width
pub const GRID_EVALUATION_STEP: f32 = 1.0 / 30.0;

/// Game window constants
pub const WINDOW_WIDTH: f32 = 1280.;
pub const WINDOW_HEIGHT: f32 = 720.;
pub const BACKGROUND_WIDTH: f32 = 6020.;
pub const BACKGROUND_HEIGHT: f32 = 3920.;

/// Animation constants
pub const ANIMATION_STEP: f32 = 0.15;

/// Player constants
// starting coordinates of player
pub const PLAYER_START_X: f32 = -300.0;
pub const PLAYER_START_Y: f32 = -200.0;

/// Gladiator constants
pub const MOVEMENT_STEP: f32 = 1.0 / 60.0; // warning, this is related to GLADIATOR_SPEED
pub const N_GLADIATORS: i32 = 5;
pub const GLADIATOR_SPEED: usize = 4; // warning, this is related to MOVEMENT_STEP
pub const GLADIATOR_SIZE: f32 = 2.5; // this scales the size of the sprite - lower once there are many
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
pub struct CombatStatus {
    engaged: bool,
}

#[derive(Component)]
pub struct Unengaged;
