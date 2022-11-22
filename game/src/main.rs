use bevy::{prelude::*, window::PresentMode};

use crate::animation::*;
use crate::gladiator::*;
use crate::player::*;
use game_lib::*;

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
        .add_plugin(PlayerPlugin)
        .add_plugin(GladiatorPlugin)
        .add_startup_system(setup)
        .add_system(animate_sprite)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());
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
