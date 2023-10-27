use std::io::Write;
use bevy::prelude::*;
use rand::{thread_rng, Rng};
#[derive(Component, Clone, Copy)]
struct Player;

#[derive(Component)]
struct Particle{
    x: f32,
    y: f32,
    life: f32
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
    mut commands: Commands,
    mut query: Query<&mut Transform, With<Player>>,
    input: Res<Input<KeyCode>>,
    time: Res<Time>
) {
    let mut transform = query.iter_mut().next().unwrap();
    if input.pressed(KeyCode::Left)   { transform.translation.x -= 3. }
    if input.pressed(KeyCode::Right)  { transform.translation.x += 3. }
    if input.pressed(KeyCode::Down)   { transform.translation.y -= 3. }
    if input.pressed(KeyCode::Up)     { transform.translation.y += 3. }

    
    if input.pressed(KeyCode::Space) { 
        println!("particle spawn! * 100");
        for i in 0..100{
            let mut rng = thread_rng();
            let (rx, ry): (f32, f32) = (rng.gen_range(0.0..10.0) - 5., rng.gen_range(0.0..10.0) - 5.);
            commands.spawn(SpriteBundle {
                sprite: Sprite {
                    color: Color::Rgba { red: 0., green: 0., blue: 0., alpha: 0.5 },
                    custom_size: Some(Vec2::new(10., 10.)),
                    
                    ..Default::default()
                },
                transform: Transform{
                    translation: Vec3::new(transform.translation.x, transform.translation.y, 0.),
                    ..Default::default()
                },  
                ..Default::default()
            }).insert(Particle{x: rx, y: ry, life: (rx.abs() + ry.abs()) * 1000.});
        }
     }
}


fn particle_movement(
    mut query: Query<(&mut Transform, &mut Particle), With<Particle>>,
    input: Res<Input<KeyCode>>,
    time: Res<Time>
) {
    for (mut transform, mut particle) in &mut query{
        transform.translation.x += particle.x;
        transform.translation.y += particle.y;
        particle.life -= 1.;
        if particle.life <= 1. {
            drop(particle)
        }
    }
    // if input.pressed(KeyCode::Space)  {  }
}
