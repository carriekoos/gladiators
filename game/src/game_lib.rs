use bevy::prelude::*;

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
pub struct Player;

#[derive(Component)]
pub struct Gladiator;

#[derive(Component)]
pub struct Movement {
    pub speed: usize,
}

// for some reason I couldn't import this guy.
#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(Timer);

#[derive(Component)]
pub struct Animation {
    /// Describes which animation is happening
    pub animation_type: AnimationType,
    /// Describes the direction that the sprite is facing
    pub animation_direction: AnimationDirection,
    /// Describes which frame of the series of images comprising an animation
    pub frame_index: usize,
}

impl Animation {
    pub fn get_sprite_index(&mut self) -> usize {
        let row_idx = self.animation_direction as usize;
        let (start, end) = self.animation_type.get_animation_type_indices();

        if self.frame_index > end - start {
            self.frame_index = 0;
            println!("other spot frame_index: {}", self.frame_index);
        }

        let col_idx = start + self.frame_index;

        (row_idx * 24) + col_idx
    }
}

pub enum AnimationType {
    Idle,
    Walk,
    Sword,
    Bow,
    Staff,
    Throw,
    Hurt,
    Death,
}

impl AnimationType {
    /// Returns the sprite animation indices from the sprite sheet row
    pub fn get_animation_type_indices(&self) -> (usize, usize) {
        match self {
            Self::Idle => (0, 1),
            Self::Walk => (2, 3),
            Self::Sword => (4, 7),
            Self::Bow => (8, 11),
            Self::Staff => (12, 14),
            Self::Throw => (15, 17),
            Self::Hurt => (18, 20),
            Self::Death => (21, 23),
        }
    }
}

#[derive(Clone, Copy)]
pub enum AnimationDirection {
    Down = 0,
    DownRight = 1,
    Right = 2,
    UpRight = 3,
    Up = 4,
    UpLeft = 5,
    Left = 6,
    DownLeft = 7,
}

impl AnimationDirection {
    pub fn from_movement(x_movement: i32, y_movement: i32) -> Result<Self, String> {
        match (x_movement, y_movement) {
            (1, 1) => Ok(AnimationDirection::UpRight),
            (1, 0) => Ok(AnimationDirection::Right),
            (1, -1) => Ok(AnimationDirection::DownRight),
            (0, 1) => Ok(AnimationDirection::Up),
            (0, 0) => Ok(AnimationDirection::Down),
            (0, -1) => Ok(AnimationDirection::Down),
            (-1, 1) => Ok(AnimationDirection::UpLeft),
            (-1, 0) => Ok(AnimationDirection::Left),
            (-1, -1) => Ok(AnimationDirection::DownLeft),
            _ => Err(
                "Movement was not a unit vector and could not generate animation direction.".into(),
            ),
        }
    }
}

/// BUNDLES
/// These are simply collections of components for organizational purposes.
#[derive(Bundle)]
pub struct PlayerBundle {
    player: Player,
    gladiator: Gladiator,
    movement: Movement,
    animation: Animation,
    animation_timer: AnimationTimer,
}

impl PlayerBundle {
    pub fn new() -> Self {
        // the default values of the animation will quickly get overwritten

        Self {
            player: Player,
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
