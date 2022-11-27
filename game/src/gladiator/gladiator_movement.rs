use bevy::prelude::*;
use rand::{self, distributions::WeightedIndex, prelude::*, rngs::ThreadRng};

use crate::{
    animation::*,
    engagements::*,
    gladiator::gladiator_components::*,
    grid::*,
    player::player_components::*,
    *, // game_lib
};

/// Moves gladiators not controlled by the player
/// TODO we can have a disjoint query here. One that is With<Engagement>
/// and the other that is Without<Engagement> and handle the movement
/// differently.
/// It would be helpful to break this out into two functions to do that.
/// For now just going to filter query to remove engaged Gladiators
pub fn gladiator_movement(
    mut query: Query<
        (&mut Transform, &Movement, &mut Animation, Entity),
        (With<Gladiator>, Without<Player>, Without<Engagement>),
    >,
    mut ev_grid_change: EventWriter<GridChangeEvent>,
) {
    for (mut transform, movement, mut animation, entity) in &mut query {
        // initialize animation type if switching from another animation.
        if !matches!(&animation.animation_type, &AnimationType::Walk) {
            animation.animation_type = AnimationType::Walk;
            animation.frame_index = 0;
        }

        let mut direction_probability = GladiatorDirectionProbability::new();

        // determine previous direction to weight the probability.
        let prev_direction_weight = 100;
        match animation.animation_direction {
            GladiatorDirection::Down => direction_probability.down = prev_direction_weight,
            GladiatorDirection::DownRight => {
                direction_probability.down_right = prev_direction_weight
            }
            GladiatorDirection::Right => direction_probability.right = prev_direction_weight,
            GladiatorDirection::UpRight => direction_probability.up_right = prev_direction_weight,
            GladiatorDirection::Up => direction_probability.up = prev_direction_weight,
            GladiatorDirection::UpLeft => direction_probability.up_left = prev_direction_weight,
            GladiatorDirection::Left => direction_probability.left = prev_direction_weight,
            GladiatorDirection::DownLeft => direction_probability.down_left = prev_direction_weight,
        }

        let new_direction = direction_probability.get_direction();
        let (mut x_movement, mut y_movement) = new_direction.to_movement();
        animation.animation_direction = new_direction;

        // maintain either left or right, otherwise default to left
        // This movement is just a placeholder until they get path planning.
        // let mut x_movement: i16 = -1;
        // (animation.animation_direction, x_movement) = match animation.animation_direction {
        //     GladiatorDirection::Down => (GladiatorDirection::Left, -1),
        //     GladiatorDirection::DownRight => (GladiatorDirection::Left, -1),
        //     GladiatorDirection::Right => (GladiatorDirection::Right, 1),
        //     GladiatorDirection::UpRight => (GladiatorDirection::Left, -1),
        //     GladiatorDirection::Up => (GladiatorDirection::Left, -1),
        //     GladiatorDirection::UpLeft => (GladiatorDirection::Left, -1),
        //     GladiatorDirection::Left => (GladiatorDirection::Left, -1),
        //     GladiatorDirection::DownLeft => (GladiatorDirection::Left, -1),
        // };

        // if too far left, go right
        if (transform.translation[0] + x_movement) < (-WINDOW_WIDTH / 2.) {
            animation.animation_direction = GladiatorDirection::Right;
            x_movement = 1.0;
        }

        // if too far right, go left
        if (transform.translation[0] + x_movement) > (WINDOW_WIDTH / 2.) {
            animation.animation_direction = GladiatorDirection::Left;
            x_movement = -1.0;
        }

        // if too far down, go up
        if (transform.translation[1] + y_movement) < (-WINDOW_HEIGHT / 2.) {
            animation.animation_direction = GladiatorDirection::Up;
            y_movement = 1.0;
        }

        // if too far up, go down
        if (transform.translation[1] + y_movement) > (WINDOW_HEIGHT / 2.) {
            animation.animation_direction = GladiatorDirection::Left;
            y_movement = -1.0;
        }

        // determine previous grid location
        let prev_grid_location =
            ArenaGrid::get_grid_location(transform.translation[0], transform.translation[1]);

        // apply the movement
        let translation_delta =
            Vec3::new(x_movement.into(), y_movement.into(), 0.0) * movement.speed;
        transform.translation += translation_delta;

        // determine current grid location
        let current_grid_location =
            ArenaGrid::get_grid_location(transform.translation[0], transform.translation[1]);

        // emit event if entering a new grid location
        if current_grid_location != prev_grid_location {
            ev_grid_change.send(GridChangeEvent {
                entity,
                prev_loc: prev_grid_location,
                curr_loc: current_grid_location,
            });
        }
    }
}

///////////////////////////////////////////////////////
/// Structs and Enums
///////////////////////////////////////////////////////

#[derive(Clone, Copy)]
pub enum GladiatorDirection {
    Down = 0,
    DownRight = 1,
    Right = 2,
    UpRight = 3,
    Up = 4,
    UpLeft = 5,
    Left = 6,
    DownLeft = 7,
}

impl GladiatorDirection {
    pub fn from_movement(x_movement: i32, y_movement: i32) -> Result<Self, String> {
        match (x_movement, y_movement) {
            (1, 1) => Ok(GladiatorDirection::UpRight),
            (1, 0) => Ok(GladiatorDirection::Right),
            (1, -1) => Ok(GladiatorDirection::DownRight),
            (0, 1) => Ok(GladiatorDirection::Up),
            (0, 0) => Ok(GladiatorDirection::Down),
            (0, -1) => Ok(GladiatorDirection::Down),
            (-1, 1) => Ok(GladiatorDirection::UpLeft),
            (-1, 0) => Ok(GladiatorDirection::Left),
            (-1, -1) => Ok(GladiatorDirection::DownLeft),
            _ => Err(
                "Movement was not a unit vector and could not generate animation direction.".into(),
            ),
        }
    }

    pub fn to_movement(&self) -> (f32, f32) {
        match self {
            GladiatorDirection::Down => (0., -1.),
            GladiatorDirection::DownRight => (1., -1.),
            GladiatorDirection::Right => (1., 0.),
            GladiatorDirection::UpRight => (1., 1.),
            GladiatorDirection::Up => (0., 1.),
            GladiatorDirection::UpLeft => (-1., 1.),
            GladiatorDirection::Left => (-1., 0.),
            GladiatorDirection::DownLeft => (-1., -1.),
        }
    }
}

struct GladiatorDirectionProbability {
    pub down: u32,
    pub down_right: u32,
    pub right: u32,
    pub up_right: u32,
    pub up: u32,
    pub up_left: u32,
    pub left: u32,
    pub down_left: u32,
    categorical_distribution: WeightedIndex<u32>,
    directions: Vec<GladiatorDirection>,
    rng: ThreadRng,
}

impl GladiatorDirectionProbability {
    pub fn new() -> Self {
        let directions = vec![
            GladiatorDirection::Down,
            GladiatorDirection::DownRight,
            GladiatorDirection::Right,
            GladiatorDirection::UpRight,
            GladiatorDirection::Up,
            GladiatorDirection::UpLeft,
            GladiatorDirection::Left,
            GladiatorDirection::DownLeft,
        ];

        // bad default
        let categorical_distribution = WeightedIndex::new(&[1, 1, 1, 1, 1, 1, 1, 1]).unwrap();
        Self {
            down: 1,
            down_right: 1,
            right: 1,
            up_right: 1,
            up: 1,
            up_left: 1,
            left: 1,
            down_left: 1,
            categorical_distribution,
            directions,
            rng: rand::thread_rng(),
        }
    }

    pub fn update_categorical_distribution(&mut self) {
        self.categorical_distribution = WeightedIndex::new(&[
            self.down,
            self.down_right,
            self.right,
            self.up_right,
            self.up,
            self.up_left,
            self.left,
            self.down_left,
        ])
        .unwrap();
    }

    pub fn get_direction(&mut self) -> GladiatorDirection {
        self.update_categorical_distribution();
        // let mut rng = rand::thread_rng();
        self.directions[self.categorical_distribution.sample(&mut self.rng)]
    }
}
