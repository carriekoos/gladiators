use bevy::prelude::*;

pub mod animation;
pub mod engagements;
pub mod gladiator;
pub mod grid;
pub mod helper_functions;
pub mod player;

// TODO several of these constsants could be pushed into a lazy static to deal with their coupling.
// probably several of them could be calculated based on the number of gladiators relative to the
// window size.

/// Grid constants
pub const GRID_HORIZONTAL_DIVISIONS: f32 = 24.; // vertical divisions calculated by this * (window) height/width
pub const GRID_EVALUATION_STEP: f32 = 1.0 / 30.0;

/// Game window constants
pub const WINDOW_WIDTH: f32 = 1280.;
pub const WINDOW_HEIGHT: f32 = 720.;
pub const BACKGROUND_WIDTH: f32 = 6020.;
pub const BACKGROUND_HEIGHT: f32 = 3920.;
pub const CLEAR: Color = Color::rgb(0.1, 0.1, 0.1);

/// Animation constants
pub const ANIMATION_STEP: f32 = 0.15;

/// Player constants
// starting coordinates of player
pub const PLAYER_START_X: f32 = -200.0;
pub const PLAYER_START_Y: f32 = -100.0;

/// Gladiator constants
pub const ATTACK_STEP: f32 = 0.3; // how fast do gladiators attack (in seconds)
pub const MOVEMENT_STEP: f32 = 1.0 / 60.0; // warning, this is related to GLADIATOR_SPEED
pub const N_GLADIATORS: usize = 300; // for now this can't be larger than the length of GLADIATOR_SPRITES
pub const GLADIATOR_SPEED: f32 = 2.; // warning, this is related to MOVEMENT_STEP and GLADIATOR_SIZE
pub const GLADIATOR_SIZE: f32 = 1.5; // this scales the size of the sprite() - lower once there are many
pub const GLADIATOR_SPRITES_PATH: &str = "Puny-Characters/";
// Just starting with a few of the sprites initially
pub const GLADIATOR_SPRITES: [&str; 5] = [
    "Soldier-Blue.png",
    "Soldier-Red.png",
    "Soldier-Yellow.png",
    "Archer-Green.png",
    "Archer-Purple.png",
];
