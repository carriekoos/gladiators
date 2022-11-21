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
                        width: 1280.,
                        height: 720.,
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
        transform: Transform::from_scale(Vec3::new(0.2126, 0.18367, 0.0)), // so not how to do this.
        ..Default::default()
    });
}
