use bevy::prelude::*;
use rand::{thread_rng, Rng};
#[derive(Component, Clone)]
struct Player{
    potion_cooldown: Timer
}

#[derive(Component)]
struct Particle{
    x: f32,
    y: f32,
    life: Timer
}

#[derive(Component)]
struct Potion{
    boom: Timer,
    x: f32,
    y: f32
}


fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, player_movement)
        .add_systems(Update, particle_movement)
        .add_systems(Update, particle_lifetime)
        .add_systems(Update,  potion_movement)
        .add_systems(Update, potion_boom)
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
    }).insert(Player {
        potion_cooldown: Timer::from_seconds(0.0, TimerMode::Once)
    });
}

fn player_movement(
    mut commands: Commands,
    mut query: Query<(&mut Transform, &mut Player), With<Player>>,
    input: Res<Input<KeyCode>>,
    time: Res<Time>
) {
    let (mut transform, mut player) = query.iter_mut().next().unwrap();
    if input.pressed(KeyCode::Left)   { transform.translation.x -= 3. }
    if input.pressed(KeyCode::Right)  { transform.translation.x += 3. }
    if input.pressed(KeyCode::Down)   { transform.translation.y -= 3. }
    if input.pressed(KeyCode::Up)     { transform.translation.y += 3. }
    player.potion_cooldown.tick(time.delta());
    if input.pressed(KeyCode::Space) && player.potion_cooldown.finished() { 
        player.potion_cooldown = Timer::from_seconds(0.1, TimerMode::Once);
        commands.spawn(SpriteBundle {
            sprite: Sprite{
                color: Color::Rgba { red: 1., green: 0., blue: 0., alpha: 1. },
                custom_size: Some(Vec2::new(10., 10.)),
                ..Default::default()
            },
            transform: Transform{
                translation: Vec3::new(transform.translation.x, transform.translation.y, 0.),
                ..Default::default()
            },
            ..Default::default()
        }).insert(Potion {
            boom: Timer::from_seconds(1., TimerMode::Once),
            x: transform.translation.x,
            y: transform.translation.y
        });
     }
}


fn particle_movement(
    mut query: Query<(&mut Transform, &mut Particle, &mut Sprite), With<Particle>>
) {
    for (mut transform, particle, mut sprite) in &mut query{
        let mut rng = thread_rng();
        transform.translation.x += particle.x+rng.gen_range(-3.0..3.0);
        transform.translation.y += particle.y;
        let rgba = sprite.color.as_rgba_f32();
        sprite.color = Color::rgba(rgba[0], rgba[1], rgba[2], rgba[3] - 0.01);
    }
    // if input.pressed(KeyCode::Space)  {  }
}
 

fn particle_lifetime(
    mut commands: Commands,
    mut query: Query<(Entity, &mut Particle), With<Particle>>,
    time: Res<Time>
) {
    for (particle_entity, mut particle) in &mut query{
        particle.life.tick(time.delta());

        if particle.life.finished() {
            commands.entity(particle_entity).despawn();
        }
    }
    // if input.pressed(KeyCode::Space)  {  }
}


fn potion_movement(
    mut query: Query<(&mut Transform, &mut Potion), With<Potion>>,
    time: Res<Time>
) {
    for (mut transform, mut potion) in &mut query{
        potion.boom.tick(time.delta());
        transform.translation.x = f32::cos(potion.boom.elapsed_secs()*1.5)*-250. + potion.x + 250.;
        transform.translation.y = f32::sin(potion.boom.elapsed_secs()*3.5)*150. + potion.y;
        transform.rotate(Quat::from_rotation_z(f32::sin(potion.boom.elapsed_secs()*3.5)*150.));
        if potion.boom.finished(){
            potion.x = f32::cos(potion.boom.elapsed_secs()*1.5)*-250. + potion.x + 250.;
            potion.y = f32::sin(potion.boom.elapsed_secs()*3.5)*150. + potion.y;
        }
        // transform.translation.x += time.delta_seconds();
        // transform.translation.y += particle.y;
    }
    // if input.pressed(KeyCode::Space)  {  }
}

fn potion_boom(
    mut commands: Commands,
    mut query: Query<(Entity, &mut Potion), With<Potion>>
) {
    for (entity, potion) in &mut query{
        if potion.boom.finished(){
            commands.entity(entity).despawn();

            for _ in 0..100{
                let mut rng = thread_rng();
                let (rx, ry): (f32, f32) = (rng.gen_range(0.0..10.0) - 5., rng.gen_range(0.0..10.0));
                commands.spawn(SpriteBundle {
                    sprite: Sprite {
                        color: Color::Rgba { red: 0., green: 0., blue: 0., alpha: 0.5 },
                        custom_size: Some(Vec2::new(10., 10.)),
                        
                        ..Default::default()
                    },
                    transform: Transform{
                        translation: Vec3::new(potion.x, potion.y, 0.),
                        ..Default::default()
                    },  
                    ..Default::default()
                }).insert(Particle{x: rx, y: ry, life: Timer::from_seconds(2., TimerMode::Once)});
            }
        }
    }
}
