use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts, EguiPlugin};

pub struct HelloPlugin;

impl Plugin for HelloPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GreetTimer(Timer::from_seconds(2.0, TimerMode::Repeating)))
            .add_systems(Startup, add_people)
            .add_systems(Update, greet_people);
    }
}

#[derive(Component)]
struct Person;

#[derive(Component)]
struct Name(String);

#[derive(Resource)]
struct GreetTimer(Timer);

fn add_people(mut commands: Commands) {
    commands.spawn((Person, Name("Foo".to_string())));
    commands.spawn((Person, Name("Bar".to_string())));
    commands.spawn((Person, Name("Baz".to_string())));
}

fn greet_people(time: Res<Time>, mut timer: ResMut<GreetTimer>, query: Query<&Name, With<Person>>) {
    if timer.0.tick(time.delta()).just_finished() {
        for name in &query {
            println!("hello {}.", name.0);
        }
    }
}

fn ui_example_system(mut contexts: EguiContexts){
    egui::Window::new("Hello").show(contexts.ctx_mut(), |ui| {
        ui.label("world");
    });
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        // .add_plugins(HelloPlugin)
        .add_plugins(EguiPlugin)
        .add_systems(Update, ui_example_system)
        .run();
}
