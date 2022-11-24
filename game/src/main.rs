use bevy::{prelude::*, window::PresentMode};
use game_lib::engagements::EngagementManagerPlugin;
use game_lib::grid::GridPlugin;

use crate::animation::*;
use crate::gladiator::*;
use crate::player::*;
use game_lib::*;

/// Road Map (in no particular order)
/// 1. Pause menu/button (space) - rename character menu
/// 2. Path planning system for Gladiators
/// 3. Gladiator engagements/debug display - DO FIRST!
/// 4. Combat system
/// 5. Death system
/// 6. Experience/leveling system
/// 7. Skills system
/// 8. Healing system
/// 9. Items system
/// 10. Start Game menu - Make easy to disable for development
/// 11. Inspectable: https://rustrepo.com/repo/jakobhellermann-bevy-inspector-egui
/// 11. Grid update event system
///   https://bevy-cheatbook.github.io/programming/events.html
///   Events seem like a many to many pub-sub system and the
///   "topics" are basically the Rust Types?
///

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
                        width: WINDOW_WIDTH,
                        height: WINDOW_HEIGHT,
                        ..default()
                    },
                    ..default()
                }),
        )
        .add_plugin(EngagementManagerPlugin)
        .add_plugin(GridPlugin)
        .add_plugin(PlayerPlugin)
        .add_plugin(GladiatorPlugin)
        .add_startup_system(setup)
        .add_system(animate_sprite)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // spawn camera
    commands.spawn(Camera2dBundle::default());

    // spawn background
    commands.spawn(SpriteBundle {
        texture: asset_server.load("arena.png"),
        transform: Transform::from_scale(
            //this. is a hack.
            Vec3::new(
                WINDOW_WIDTH / BACKGROUND_WIDTH,
                WINDOW_HEIGHT / BACKGROUND_HEIGHT,
                -900.0,
            ),
        ),
        ..Default::default()
    });
}
