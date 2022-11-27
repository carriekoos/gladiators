use bevy::{prelude::*, window::PresentMode};

use crate::animation::AnimationPlugin;
use crate::engagements::EngagementManagerPlugin;
use crate::gladiator::gladiator::GladiatorPlugin;
use crate::grid::GridPlugin;
use crate::player::player::PlayerPlugin;
use game_lib::*;

/// Road Map (in no particular order)
/// 1. Pause menu/button (space) - rename character menu
/// 2. Path planning system for Gladiators - DONE
/// 3. Gladiator engagements - DONE
/// 4. Combat system - DONE
/// 5. Death system - DONE
/// 6. Experience/leveling system
/// 7. Skills system
/// 8. Healing system
/// 9. Items system
/// 10. Start Game menu - Make easy to disable for development
/// 11. Inspectable: https://rustrepo.com/repo/jakobhellermann-bevy-inspector-egui
/// 11. Grid update event system - DONE
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
        .add_plugin(AnimationPlugin)
        .add_plugin(EngagementManagerPlugin)
        .add_plugin(GridPlugin)
        .add_plugin(PlayerPlugin)
        .add_plugin(GladiatorPlugin)
        .add_startup_system(setup)
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
