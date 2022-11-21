use bevy::{prelude::*, time::FixedTimestep, window::PresentMode};
use game_lib::*; // lol just until I figure out how to organize using plugins

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
        .insert(PlayerBundle::new());
}

fn spawn_gladiator(
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
