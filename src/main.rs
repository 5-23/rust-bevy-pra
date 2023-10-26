use std::io::Write;
use bevy::prelude::*;
use rand::Rng;
#[derive(Component, Clone, Copy)]
struct Player;

#[derive(Component)]
struct Particle{
    x: f32,
    y: f32
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, player_movement)
        .add_systems(Update, particle_movement)
        .run();
}


fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn(SpriteBundle {
        sprite: Sprite {
            custom_size: Some(Vec2::new(200., 200.)),
            ..Default::default()
        },
        texture: asset_server.load("sprite.png"),
        transform: Transform{
            ..Default::default()
        },
        ..Default::default()
    }).insert(Player);
}

fn player_movement(
    mut query: Query<&mut Transform, With<Player>>,
    input: Res<Input<KeyCode>>,
    time: Res<Time>
) {
    let mut transform = query.iter_mut().next().unwrap();
    if input.pressed(KeyCode::Left)   { transform.translation.x -= 3. }
    if input.pressed(KeyCode::Right)  { transform.translation.x += 3. }
    if input.pressed(KeyCode::Down)   { transform.translation.y -= 3. }
    if input.pressed(KeyCode::Up)     { transform.translation.y += 3. }
}


fn particle_movement(
    commands: Commands,
    mut query: Query<&mut Transform, With<Particle>>,
    input: Res<Input<KeyCode>>,
    time: Res<Time>
) {
    if input.pressed(KeyCode::Space) { 
        let rng = rand::thread_rng();
        let (rx, ry) = (rng::gen_range(0.0..10.0), rng::gen_range(0.0..10.0))
        commands.spawn(Sprite {

        }).insert(Particle{x: rx, y: ry})
     }
    for transform in &mut query{

    }
    // if input.pressed(KeyCode::Space)  {  }
}