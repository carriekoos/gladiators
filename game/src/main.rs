use bevy::{prelude::*, window::PresentMode};

use crate::animation::*;
use crate::gladiator::*;
use crate::player::*;
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
        .add_plugin(PlayerPlugin)
        .add_plugin(GladiatorPlugin)
        .add_startup_system(setup)
        .add_system(animate_sprite)
        .run();
}

fn setup(mut commands: Commands) {
    // spawn camera
    commands.spawn(Camera2dBundle::default());
}
