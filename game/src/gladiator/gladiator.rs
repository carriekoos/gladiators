use bevy::{prelude::*, time::FixedTimestep};
use rand::{self, Rng};

use crate::{
    gladiator::{
        gladiator_bundles::*, gladiator_combat::*, gladiator_events::*, gladiator_movement::*,
    },
    *, // game_lib
};

///////////////////////////////////////////////////////
/// Bugs
///////////////////////////////////////////////////////
/// thread 'main' panicked at 'Victor of engagement should exist in ECS.: NoSuchEntity(276v0)', game\src\gladiator\gladiator_combat.rs:83:14
/// 276v0 is dead!
/// 301v0 is dead!
/// 184v0 attacking 8v0 for 1 damage!
/// 8v0 attacking 184v0 for 1 damage!
/// 276v0 attacking 52v0 for 3 damage!

///////////////////////////////////////////////////////
/// Plugin
///////////////////////////////////////////////////////

pub struct GladiatorPlugin;

impl Plugin for GladiatorPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_gladiators)
            .add_system_set(
                SystemSet::new()
                    .with_run_criteria(FixedTimestep::step(MOVEMENT_STEP as f64))
                    .with_system(gladiator_movement),
            )
            .add_system(gladiator_attacks)
            .add_system(gladiator_receive_attack)
            .add_system(gladiator_death_handler)
            .add_event::<AttackEvent>()
            .add_event::<DeathEvent>();
    }
}

///////////////////////////////////////////////////////
/// Functions
///////////////////////////////////////////////////////

fn spawn_gladiators(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    for i in 0..N_GLADIATORS {
        let mut rng = rand::thread_rng();
        let x = rng.gen_range(-WINDOW_WIDTH..WINDOW_WIDTH);
        let y = rng.gen_range(-WINDOW_HEIGHT..WINDOW_HEIGHT);
        spawn_one_gladiator(
            Vec2::new(x, y),
            i as usize,
            &mut commands,
            &asset_server,
            &mut texture_atlases,
        );
    }
}

/// Spawns a gladiator not controlled by the player
fn spawn_one_gladiator(
    location: Vec2,
    gladiator_idx: usize,
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    texture_atlases: &mut ResMut<Assets<TextureAtlas>>,
) {
    let sprite_file =
        GLADIATOR_SPRITES[(gladiator_idx % GLADIATOR_SPRITES.len()) as usize].to_string();

    // just a quick hack, doesn't evenly distribute the classes because there are more
    // warrior and soldier sprite sheets
    let gladiator_class = if sprite_file.contains("Archer") {
        Class::Archer
    } else if sprite_file.contains("Mage") {
        Class::Mage
    } else if sprite_file.contains("Soldier") {
        Class::Fighter
    } else if sprite_file.contains("Warrior") {
        Class::Fighter
    } else {
        Class::Fighter
    };

    // grab a different spritesheet based on gladiator_idx
    let path = format!("{}{}", GLADIATOR_SPRITES_PATH, sprite_file);

    let texture_handle = asset_server.load(&path);
    // The values used in the next function are specific to the Puny Characters sprite sheets
    // TODO lazy static this?
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
        .insert(GladiatorBundle::new(gladiator_class));
}

///////////////////////////////////////////////////////
/// Structs and Enums
///////////////////////////////////////////////////////

pub enum Class {
    Archer,
    Mage,
    Fighter,
}
