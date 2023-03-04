use bevy::{prelude::*};

const IMG: &str = "player.png";

#[derive(Component, Debug)]
pub struct Velo{
    pub left: f32,
    pub right: f32,
    pub up: f32,
    pub down: f32
}
impl Velo{
    pub fn check(&mut self, max: f32){
        self.up    = if self.up    < 0.{ 0. } else{ self.up };
        self.down  = if self.down  < 0.{ 0. } else{ self.down };
        self.left  = if self.left  < 0.{ 0. } else{ self.left };
        self.right = if self.right < 0.{ 0. } else{ self.right };
        
        self.up    = if self.up     > max { max } else{ self.up };
        self.down  = if self.down   > max { max } else{ self.down };
        self.left  = if self.left   > max { max } else{ self.left };
        self.right = if self.right  > max { max } else{ self.right };
    }
}

pub struct PlayerPlugin;
impl Plugin for PlayerPlugin{
    fn build(&self, app: &mut App) {
        app
            .add_startup_system(setup_system)
            .add_system(movement)
            .add_system(key_env)
        ;
    }
}

#[derive(Component)]
struct Player;

fn setup_system(mut commands: Commands, asset_server: Res<AssetServer>){
    commands.spawn(SpriteBundle {
        sprite: Sprite {
            custom_size: Some(Vec2::new(100., 100.)),
            ..default()
        },
        texture: asset_server.load(IMG),
        transform: Transform{
            translation: Vec3::new(-500., 0., 0.),
            ..Default::default()
        },
        ..default()
    })
        .insert(Player)
        .insert(Velo{ up: 0., down: 0., left: 0., right: 0., })
        ;
}

fn movement(mut q: Query<(&Velo, &mut Transform), With<Player>>){
    for (velo, mut transform) in q.iter_mut(){
        let translation = &mut transform.translation;
        translation.x += velo.right - velo.left;
        translation.y += velo.up - velo.down;
    }
}

fn key_env(key: Res<Input<KeyCode>>,mut q: Query<(&mut Velo, &mut Transform), With<Player>>){
    let acc = 0.05;
    let res = 0.05; 
    let max = 4.;
    for (mut velo, _) in q.iter_mut(){
        velo.left += if key.pressed(KeyCode::Left)   { acc }else{ -res };
        velo.right += if key.pressed(KeyCode::Right) { acc }else{ -res };
        velo.up += if key.pressed(KeyCode::Up)       { acc }else{ -res };
        velo.down += if key.pressed(KeyCode::Down)   { acc }else{ -res };

        velo.check(max);
    }
}