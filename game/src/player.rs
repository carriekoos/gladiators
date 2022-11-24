use bevy::{prelude::*, time::FixedTimestep};

use crate::{animation::*, engagements::Engagement, gladiator::*, grid::*, *};

/// Just some notes about where this part of the project is going:
/// What is the advantage/strategy that the Player has over other Gladiators?
/// 1. Delay between engagements to restore health
/// 2. Go after appropriate targets - can intuit this based on size of sprite
///  if level can inform sprite scale (it so can)
/// 3. Switch weapons? - probably should be available for everyone.
/// 4. Go after items? - also maybe should be for everyone?
/// Want to make sure that we're not deliberately hamstringing the other
/// Gladiators in a weird way that doesn't make a lot of sense.
/// The AI doesn't need to be overly sophisticated, but maybe shouldn't
/// be completely stupid.

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_player).add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(MOVEMENT_STEP as f64))
                .with_system(player_movement),
        );
    }
}

/// Spawns a gladiator that is controlled by the player
fn spawn_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let texture_handle = asset_server.load("Puny-Characters/Mage-Cyan.png");
    // The values used in the next function are specific to the Puny Characters sprite sheets
    let texture_atlas = TextureAtlas::from_grid(
        texture_handle,
        Vec2::new(24.0, 24.0),
        24,
        8,
        Some(Vec2::new(8.0, 8.0)),
        None,
    );
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    // set size and starting location of the gladiator
    let mut transform = Transform::from_scale(Vec3::splat(GLADIATOR_SIZE));
    transform.translation = Vec3::new(PLAYER_START_X, PLAYER_START_Y, 2.0);

    // Note: When spawning an entity, you call commands.spawn() and then chain .insert() over and over,
    // adding additional components to that entity. Order doesn't matter.
    // You can also construct a bundle to make it easier to call .insert() once per logical concept.
    // I did this for PlayerBundle and GladiatorBundle so that I didn't have to add each of their
    // Components one by one.
    // Call spawn() again for a new entity.
    //
    // spawn player
    commands
        .spawn((SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            transform,
            ..default()
        },))
        .insert(PlayerBundle::new())
        .insert(Health {
            // override default health value for player so player is more robust for now.
            value: 999.0,
        });
}

/// Moves the gladiator controlled by the player
fn player_movement(
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
            match AnimationDirection::from_movement(x_movement.into(), y_movement.into()) {
                Ok(direction) => direction,
                Err(err) => {
                    println!(
                        "Unable to set animation direction. {} Setting to DOWN.",
                        err
                    );
                    AnimationDirection::Down
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
