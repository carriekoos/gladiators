use bevy::prelude::*;

use crate::{
    animation::*,
    engagements::Engagement,
    gladiator::{gladiator_components::*, gladiator_movement::*},
    grid::*,
    player::player_components::*,
    *, // game_lib
};

/// Moves the gladiator controlled by the player
pub fn player_movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut ev_grid_change: EventWriter<GridChangeEvent>,
    mut query: Query<
        (&mut Transform, &Movement, &mut Animation, Entity),
        (With<Player>, Without<Engagement>),
    >,
) {
    for (mut transform, movement, mut animation, entity) in &mut query {
        // get movement input
        let mut x_movement: i16 = 0;
        let mut y_movement: i16 = 0;

        if keyboard_input.pressed(KeyCode::W) || keyboard_input.pressed(KeyCode::Up) {
            y_movement += 1;
        }

        if keyboard_input.pressed(KeyCode::S) || keyboard_input.pressed(KeyCode::Down) {
            y_movement -= 1;
        }

        if keyboard_input.pressed(KeyCode::D) || keyboard_input.pressed(KeyCode::Right) {
            x_movement += 1;
        }

        if keyboard_input.pressed(KeyCode::A) || keyboard_input.pressed(KeyCode::Left) {
            x_movement -= 1;
        }

        if x_movement == 0 && y_movement == 0 {
            animation.animation_type = AnimationType::Idle
        } else {
            animation.animation_type = AnimationType::Walk
        }

        // boundary detection adjusts x and y to head away from boundary
        // too far left
        if transform.translation[0] < -WINDOW_WIDTH / 2. {
            x_movement = 1;
        }

        // too far right
        if transform.translation[0] > WINDOW_WIDTH / 2. {
            x_movement = -1;
        }

        // too far down
        if transform.translation[1] < -WINDOW_HEIGHT / 2. {
            y_movement = 1;
        }

        // too far up
        if transform.translation[1] > WINDOW_HEIGHT / 2. {
            y_movement = -1;
        }

        // adjust direction
        animation.animation_direction =
            match GladiatorDirection::from_movement(x_movement.into(), y_movement.into()) {
                Ok(direction) => direction,
                Err(err) => {
                    println!(
                        "Unable to set animation direction. {} Setting to DOWN.",
                        err
                    );
                    GladiatorDirection::Down
                }
            };

        // determine previous grid location
        let prev_grid_location =
            ArenaGrid::get_grid_location(transform.translation[0], transform.translation[1]);

        // translate
        let translation_delta =
            Vec3::new(x_movement.into(), y_movement.into(), 0.0) * movement.speed as f32;
        transform.translation += translation_delta;

        // determine current grid location
        let current_grid_location =
            ArenaGrid::get_grid_location(transform.translation[0], transform.translation[1]);

        // emit event if entering a new grid location
        if current_grid_location != prev_grid_location {
            // can replace this with Inspector plugin
            // println!(
            //     "player at: x {} | y: {}",
            //     current_grid_location.x, current_grid_location.y
            // );
            ev_grid_change.send(GridChangeEvent {
                entity,
                prev_loc: prev_grid_location,
                curr_loc: current_grid_location,
            });
        }
    }

    // For the player, I don't think that I need to do anything else.
}
