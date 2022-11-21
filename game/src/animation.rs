use bevy::prelude::*;

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
