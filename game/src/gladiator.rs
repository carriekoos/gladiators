use bevy::{prelude::*, time::FixedTimestep};

use crate::{animation::*, engagements::*, player::*, *};

pub struct GladiatorPlugin;

impl Plugin for GladiatorPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_gladiators).add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(MOVEMENT_STEP as f64))
                .with_system(gladiator_movement),
        )
        .add_system(gladiator_combat);
    }
}

fn gladiator_combat(
    &mut engagements_query: Query<&Engagement>,
    &mut gladiators_query: Query<&Gladiators, &mut Health>,
) {
    for engagement in &query {
        // now what do I do with them?
        println!("{} and {} are engaged in combat!", engagement.gladiator_a, engagement.gladiator_b);
        let a = gladiators_query.get(engagement.gladiator_a);
        do_combat(a, b);
    }
}

fn do_combat(
    a: Entity,
    b: Entity,
) {
    asdf
}

fn spawn_gladiators(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    for i in 0..N_GLADIATORS {
        let coordinate = (50 * i) as f32;
        spawn_one_gladiator(
            Vec2::splat(coordinate),
            i,
            &mut commands,
            &asset_server,
            &mut texture_atlases,
        );
    }
}

/// Spawns a gladiator not controlled by the player
fn spawn_one_gladiator(
    location: Vec2,
    gladiator_idx: i32,
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    texture_atlases: &mut ResMut<Assets<TextureAtlas>>,
) {
    // grab a different spritesheet based on gladiator_idx
    let path = format!(
        "{}{}",
        GLADIATOR_SPRITES_PATH, GLADIATOR_SPRITES[gladiator_idx as usize]
    );

    let texture_handle = asset_server.load(&path);
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
    transform.translation = location.extend(1.0);

    commands
        .spawn(SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            transform,
            ..default()
        })
        .insert(GladiatorBundle::new());
}

/// Moves gladiators not controlled by the player
fn gladiator_movement(
    mut query: Query<
        (&mut Transform, &Movement, &mut Animation),
        (With<Gladiator>, Without<Player>),
    >,
) {
    for (mut transform, movement, mut animation) in &mut query {
        animation.animation_type = AnimationType::Walk;

        // maintain either left or right, otherwise default to left
        // This movement is just a placeholder until they get path planning.
        let mut x_movement: i16 = -1;
        (animation.animation_direction, x_movement) = match animation.animation_direction {
            AnimationDirection::Down => (AnimationDirection::Left, -1),
            AnimationDirection::DownRight => (AnimationDirection::Left, -1),
            AnimationDirection::Right => (AnimationDirection::Right, 1),
            AnimationDirection::UpRight => (AnimationDirection::Left, -1),
            AnimationDirection::Up => (AnimationDirection::Left, -1),
            AnimationDirection::UpLeft => (AnimationDirection::Left, -1),
            AnimationDirection::Left => (AnimationDirection::Left, -1),
            AnimationDirection::DownLeft => (AnimationDirection::Left, -1),
        };

        // if too far left, go right
        if transform.translation[0] < -WINDOW_WIDTH / 2. {
            animation.animation_direction = AnimationDirection::Right;
            x_movement = 1;
        }

        // if too far right, go left
        if transform.translation[0] > WINDOW_WIDTH / 2. {
            animation.animation_direction = AnimationDirection::Left;
            x_movement = -1;
        }

        // apply the movement
        let y_movement: i16 = 0;
        let translation_delta =
            Vec3::new(x_movement.into(), y_movement.into(), 0.0) * movement.speed;
        transform.translation += translation_delta;
    }
}

// for some reason I couldn't import this guy.
#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(Timer);

#[derive(Component)]
pub struct Movement {
    pub speed: f32,
}

#[derive(Component)]
pub struct Gladiator;

#[derive(Component)]
pub struct Health(f32);

#[derive(Component)]
pub struct Attack(f32);

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
