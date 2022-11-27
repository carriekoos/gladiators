use bevy::{prelude::*, time::FixedTimestep};

use crate::{
    gladiator::gladiator_components::*,
    player::{player_bundles::*, player_movement::*},
    *, // game_lib
};

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

///////////////////////////////////////////////////////
/// Plugin
///////////////////////////////////////////////////////

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

///////////////////////////////////////////////////////
/// Functions
///////////////////////////////////////////////////////

/// Spawns a gladiator that is controlled by the player
// TODO instead of spawning a player, spawn a gladiator and then take control of one of them
//  as the player?
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
