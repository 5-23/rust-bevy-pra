use std::io::Write;

use bevy::prelude::*;

#[derive(Component, Debug)]
struct Pos{
    x: f32,
    y: f32
}

#[derive(Component)]
struct Player;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, just_input)
        .run();
}


fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn((Player, Pos{x: 0., y: 0.}));
    // commands.spawn(SpriteBundle {
    //     sprite: Sprite {
    //         custom_size: Some(Vec2::new(200.0, 200.0)),
    //         // color: Color::rgb(0.0, 0.0, 0.0),
    //         ..Default::default()s
    //     },
    //     texture: asset_server.load("sprite.png"),
    //     ..Default::default()
    // });
}

fn just_input(mut query: Query<&mut Pos, With<Player>>, input: Res<Input<KeyCode>>, time: Res<Time>) {
    if input.pressed(KeyCode::Left) { query.single_mut().x -= 1. }
    else if input.pressed(KeyCode::Right) { query.single_mut().x += 1. }
    else if input.pressed(KeyCode::Down) { query.single_mut().y += 1. }
    else if input.pressed(KeyCode::Up) { query.single_mut().y -= 1. }
    else {}
    println!("{:?} ", query.single());
}