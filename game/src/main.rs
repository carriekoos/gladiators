use bevy::{prelude::*, render::texture, time::FixedTimestep, window::PresentMode};
use leafwing_input_manager::prelude::*;

pub const CLEAR: Color = Color::rgb(0.1, 0.1, 0.1);
pub const ANIMATION_STEP: f32 = 0.15;
pub const MOVEMENT_STEP: f32 = 1.0 / 60.0; // warning, this is related to PLAYER_SPEED
pub const GLADIATOR_SPEED: usize = 4; // warning, this is related to MOVEMENT_STEP
pub const GLADIATOR_SIZE: f32 = 4.0; // this scales the size of the sprite - lower once there are many

#[derive(Component, Deref, DerefMut)]
struct AnimationTimer(Timer);

#[derive(Component)]
struct Player;

#[derive(Component)]
struct Gladiator;

#[derive(Component)]
struct Movement {
    speed: usize,
}

#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug)]
enum Action {
    Move,
}

#[derive(Component)]
pub struct Animation {
    /// Describes which animation is happening
    animation_type: AnimationType,
    /// Describes the direction that the sprite is facing
    animation_direction: AnimationDirection,
    /// Describes which frame of the series of images comprising an animation
    frame_index: usize,
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

/// This is the main function that runs the game.
fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    window: WindowDescriptor {
                        title: "Gladiators".into(),
                        present_mode: PresentMode::Fifo,
                        resizable: false,
                        ..default()
                    },
                    ..default()
                }),
        )
        .add_plugin(InputManagerPlugin::<Action>::default())
        .add_startup_system(setup)
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(MOVEMENT_STEP as f64))
                .with_system(gladiator_movement)
                .with_system(player_movement),
        )
        .add_system(animate_sprite)
        .run();
}

// TODO figure out how to refactor setup into a plugin
fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    // spawn camera
    commands.spawn(Camera2dBundle::default());

    // spawn player
    spawn_player(
        Vec2::new(-200.0, -200.0),
        &mut commands,
        &asset_server,
        &mut texture_atlases,
    );

    //spawn other gladiators
    const N_GLADIATORS: i32 = 5;
    for i in 0..N_GLADIATORS {
        let coordinate = (50 * i) as f32;
        spawn_gladiator(
            Vec2::splat(coordinate),
            i,
            &mut commands,
            &asset_server,
            &mut texture_atlases,
        );
    }
}

fn spawn_player(
    location: Vec2,
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    texture_atlases: &mut ResMut<Assets<TextureAtlas>>,
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
    transform.translation = location.extend(2.0);

    // Note: When spawning an entity, you call commands.spawn() and then chain .insert() over and over,
    // adding additional components to that entity. Order doesn't matter.
    // Call spawn() again for a new entity.
    //
    // spawn player
    commands
        .spawn((
            SpriteSheetBundle {
                texture_atlas: texture_atlas_handle,
                transform,
                ..default()
            },
            AnimationTimer(Timer::from_seconds(ANIMATION_STEP, TimerMode::Repeating)),
        ))
        .insert(Player)
        .insert(Gladiator)
        .insert(Movement {
            speed: GLADIATOR_SPEED,
        })
        .insert(Animation {
            animation_type: AnimationType::Idle,
            animation_direction: AnimationDirection::Down,
            frame_index: 0,
        });
}

fn spawn_gladiator(
    location: Vec2,
    gladiator_idx: i32,
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    texture_atlases: &mut ResMut<Assets<TextureAtlas>>,
) {
    let gladiator_sprites = [
        "Soldier-Blue.png",
        "Soldier-Red.png",
        "Soldier-Yellow.png",
        "Archer-Green.png",
        "Archer-Purple.png",
    ];
    let path = format!(
        "Puny-Characters/{}",
        gladiator_sprites[gladiator_idx as usize]
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
        .spawn((
            SpriteSheetBundle {
                texture_atlas: texture_atlas_handle,
                transform,
                ..default()
            },
            AnimationTimer(Timer::from_seconds(ANIMATION_STEP, TimerMode::Repeating)),
        ))
        .insert(Gladiator)
        .insert(Movement {
            speed: GLADIATOR_SPEED,
        })
        .insert(Animation {
            animation_type: AnimationType::Idle,
            animation_direction: AnimationDirection::Down,
            frame_index: 0,
        });
}

fn gladiator_movement(
    mut query: Query<
        (&mut Transform, &Movement, &mut Animation),
        (With<Gladiator>, Without<Player>),
    >,
) {
    for (mut transform, movement, mut animation) in &mut query {
        animation.animation_type = AnimationType::Walk;

        // maintain either left or right, otherwise default to left
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
        if transform.translation[0] < -300.0 {
            animation.animation_direction = AnimationDirection::Right;
            x_movement = 1;
        }

        // if too far right, go left
        if transform.translation[0] > 300.0 {
            animation.animation_direction = AnimationDirection::Left;
            x_movement = -1;
        }

        // apply the movement
        let y_movement: i16 = 0;
        let translation_delta =
            Vec3::new(x_movement.into(), y_movement.into(), 0.0) * movement.speed as f32;
        transform.translation += translation_delta;
    }
}

fn player_movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&mut Transform, &Movement, &mut Animation), With<Player>>,
) {
    // we know single_mut() works because we're only spawning one player right now.
    let (mut transform, movement, mut animation) = query.single_mut();

    // get movement input
    let mut x_movement: i16 = 0;
    let mut y_movement: i16 = 0;

    if keyboard_input.pressed(KeyCode::W) {
        y_movement += 1;
    }

    if keyboard_input.pressed(KeyCode::S) {
        y_movement -= 1;
    }

    if keyboard_input.pressed(KeyCode::D) {
        x_movement += 1;
    }

    if keyboard_input.pressed(KeyCode::A) {
        x_movement -= 1;
    }

    if x_movement == 0 && y_movement == 0 {
        animation.animation_type = AnimationType::Idle
    } else {
        animation.animation_type = AnimationType::Walk
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

    // translate
    let translation_delta =
        Vec3::new(x_movement.into(), y_movement.into(), 0.0) * movement.speed as f32;
    transform.translation += translation_delta;
}

fn animate_sprite(
    time: Res<Time>,
    mut query: Query<
        (
            // give me all the entities
            &mut AnimationTimer,     // with an AnimationTimer,
            &mut TextureAtlasSprite, // a TextureAtlasSprite,
            &mut Animation,
        ),
        With<Gladiator>,
    >,
) {
    // single_mut() works right now because we only have one Player entity
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
