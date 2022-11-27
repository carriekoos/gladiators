use bevy::{prelude::*, time::FixedTimestep};
use rand::{self, Rng};

use crate::{
    gladiator::{
        gladiator_bundles::*, gladiator_combat::*, gladiator_events::*, gladiator_movement::*,
    },
    *, // game_lib
};

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

/// We could set up the engagement builder to tag gladiators as engaged with the Entity
/// of their target. Then combat is done by looping through all gladiators that are engaged
/// and emitting AttackEvents that contain the target entity. Call this gladiator_attacks().
/// The timing of attacks can differ per gladiator here by having a Timer, similar to the
/// animation timer
/// A separate system here has an event reader that loops through all of the attack events,
/// applying the damage to the health based on the defense. In this service, if the health
/// is dropped below zero, we can emit a DeathEvent. The death event will send the level
/// of the gladiator that died (used for rewarding experience) as well as the Entity who
/// should recieve the experience. In order to have this data, the attacker Entity will
/// need to be data contained in the AttackEvents. Death events will despawn the dead
/// Gladiator and the Gladiator that receives the experience will despawn the Engagement
/// on itself so that they become an eligible bachelor yet again.
// fn gladiator_combat(
//     engagement_query: Query<&Engagement>,
//     mut gladiator_query: Query<(&mut Health, &mut Level, &Attack, &Defense), With<Gladiator>>,
// ) {
//     for engagement in &engagement_query {
//         let (mut health_a, mut level_a, attack_a, defense_a) = gladiator_query
//             .get_mut(engagement.gladiator_a)
//             .expect("A gladiator in an engagement should exist in the ECS");
//         let (mut health_b, mut level_b, attack_b, defense_b) = gladiator_query
//             .get_mut(engagement.gladiator_b)
//             .expect("A gladiator in an engagement should exist in the ECS");

//         // TODO So I can't do both querys at the same time with mutable reference. I could just get the data, perform logic, returning the new data, then
//         //  get mutable reference again to one at a time mutate, bounded by little mini scopes. hmmmm, don't love it.
//         // do_combat(
//         //     health_a,
//         //     level_a,
//         //     attack_a,
//         //     defense_a,
//         //     &mut *health_b,
//         //     &mut *level_b,
//         //     attack_b,
//         //     defense_b,
//         // );
//     }
// }

// fn do_combat(
//     &mut health_a: &mut Health,
//     level_a: &mut Level,
//     attack_a: &Attack,
//     defense_a: &Defense,
//     health_b: &mut Health,
//     level_b: &mut Level,
//     attack_b: &Attack,
//     defense_b: &Defense,
// ) {
//     // do stuff
// }

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
