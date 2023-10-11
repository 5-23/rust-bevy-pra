use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, start)
        .add_plugins(TimerPlugin)
        .run();
}

#[derive(Debug, Component)]
struct Entity {
    id: usize,
    name: String
}

impl Entity {
    fn new(id: usize, name: String) -> Self {
        Self { id, name }
    }
}

fn start(mut cmd: Commands) {
    cmd.spawn(Entity::new(1, "5-23".to_string()));
    cmd.spawn(Entity::new(2, "5-24".to_string()));
}

#[derive(Resource)]
struct GreetTimer(Timer);

fn greet_people(
    time: Res<Time>, mut timer: ResMut<GreetTimer>, query: Query<&Entity>) {
    // update our timer with the time elapsed since the last update
    // if that caused the timer to finish, we say hello to everyone
    if timer.0.tick(time.delta()).just_finished() {
        for e in &query {
            println!("{e:?}");
        }
    }
}

struct TimerPlugin;

impl Plugin for TimerPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GreetTimer(Timer::from_seconds(2.0, TimerMode::Repeating)))
            .add_systems(Update, greet_people);
    }
}