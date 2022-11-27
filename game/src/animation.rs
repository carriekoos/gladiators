use bevy::prelude::*;

use crate::gladiator::{gladiator::*, gladiator_components::*};

///////////////////////////////////////////////////////
/// Plugin
///////////////////////////////////////////////////////

pub struct AnimationPlugin;

impl Plugin for AnimationPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(animate_sprites);
    }
}

///////////////////////////////////////////////////////
/// Functions
///////////////////////////////////////////////////////

/// TODO we can have disjoint queries here for With/Without<Engagement>
/// With<> is going to do attack animations that could look at other data
/// such as EquippedWeapon or something. Without<> is going to perform
/// the movement logic already present here.
pub fn animate_sprites(
    time: Res<Time>,
    mut query: Query<
        (
            // give me all the entities
            &mut AnimationTimer,
            &mut TextureAtlasSprite,
            &mut Animation,
        ),
        With<Gladiator>,
    >,
) {
    for (mut timer, mut sprite, mut animation) in &mut query {
        timer.tick(time.delta());
        if timer.just_finished() {
            sprite.index = animation.get_sprite_index(); // this can change it to 0
            animation.frame_index += 1; // but then without _seeing_ 0, it gets incremented
            let (start, end) = animation.animation_type.get_animation_type_indices();

            if animation.frame_index > end - start {
                animation.frame_index = 0;
            }
        }
    }
}

///////////////////////////////////////////////////////
/// Structs and Enums
///////////////////////////////////////////////////////

#[derive(Component)]
pub struct Animation {
    /// Describes which animation is happening
    pub animation_type: AnimationType,
    /// Describes the direction that the sprite is facing
    pub animation_direction: GladiatorDirection,
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
