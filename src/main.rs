use std::io::Write;

use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .run();
}


fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn(SpriteBundle {
        sprite: Sprite {
            custom_size: Some(Vec2::new(200.0, 200.0)),
            // color: Color::rgb(0.0, 0.0, 0.0),
            ..Default::default()
        },
        texture: asset_server.load("sprite.png"),
        ..Default::default()
    });
}