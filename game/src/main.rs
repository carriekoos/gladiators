use bevy::{prelude::*, time::FixedTimestep, window::PresentMode};
use leafwing_input_manager::prelude::*;

pub const CLEAR: Color = Color::rgb(0.1, 0.1, 0.1);
pub const ANIMATION_STEP: f32 = 0.15;
pub const MOVEMENT_STEP: f32 = 1.0 / 60.0; // warning, this is related to PLAYER_SPEED
pub const PLAYER_SPEED: usize = 4; // warning, this is related to MOVEMENT_STEP
                                   // pub const MOVEMENT_STEP: f32 = 1.0 / 60.0;

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
                .with_system(movement),
        )
        .add_system(animate_sprite)
        .run();
}

#[derive(Component, Deref, DerefMut)]
struct AnimationTimer(Timer);

fn animate_sprite(
    time: Res<Time>,
    mut query: Query<
        (
            // give me all the entities
            &mut AnimationTimer,     // with an AnimationTimer,
            &mut TextureAtlasSprite, // a TextureAtlasSprite,
            &mut Animation,
        ),
        With<Player>,
    >,
) {
    // single_mut() works right now because we only have one Player entity
    let (mut timer, mut sprite, mut animation) = query.single_mut();
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

#[derive(Component)]
struct Player;

#[derive(Component)]
struct Movement {
    speed: usize,
}

#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug)]
enum Action {
    Move,
}

fn movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&mut Transform, &Movement, &mut Animation), With<Player>>,
) {
    // we know single_mut() works because we're only spawning one player initially.
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
    let translation_delta: Vec3 =
        Vec3::new(x_movement.into(), y_movement.into(), 0.0) * movement.speed as f32;
    transform.translation += translation_delta;
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    // spawn camera
    commands.spawn(Camera2dBundle::default());

    // Note: When spawning an entity, you call commands.spawn() and then chain .insert() over and over,
    // adding additional components to that entity. Order doesn't matter.
    // Call spawn() again for a new entity.
    //
    // spawn player
    let texture_handle = asset_server.load("Puny-Characters/Mage-Cyan.png");
    let texture_atlas = TextureAtlas::from_grid(
        texture_handle,
        Vec2::new(24.0, 24.0),
        24,
        8,
        Some(Vec2::new(8.0, 8.0)),
        None,
    );
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    commands
        .spawn((
            SpriteSheetBundle {
                texture_atlas: texture_atlas_handle,
                transform: Transform::from_scale(Vec3::splat(4.0)), // this scales the size of the sprite
                ..default()
            },
            AnimationTimer(Timer::from_seconds(ANIMATION_STEP, TimerMode::Repeating)),
        ))
        .insert(Player)
        .insert(Movement {
            speed: PLAYER_SPEED,
        })
        .insert(Animation {
            animation_type: AnimationType::Idle,
            animation_direction: AnimationDirection::Down,
            frame_index: 0,
        })
        .insert(InputManagerBundle::<Action> {
            action_state: ActionState::default(),
            input_map: InputMap::default()
                .insert(DualAxis::left_stick(), Action::Move)
                .set_gamepad(Gamepad { id: 0 })
                .build(),
        });
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
