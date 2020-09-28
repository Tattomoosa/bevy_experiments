use bevy::prelude::*;

struct Person;

#[derive(Debug)]
struct Name(String);

fn add_people(mut commands: Commands) {
    commands
        .spawn((Person, Name("Matt O".to_string())))
        .spawn((Person, Name("Lys G".to_string())));
}

struct GreetTimer(Timer);


/*
// "foreach" system - bug where timer is updated
// this will run once per cycle for every entity (Person, Name)
fn greet_people(
    time: Res<Time>,
    mut timer: ResMut<GreetTimer>,
    _person: &Person, name: &Name
) {
    timer.0.tick(time.delta_seconds);
    if timer.0.finished {
        println!("hello {}", name.0);
    }
}
*/

// query system
// this runs once per cycle and handles all entities (Person, Name)
fn greet_people(
    time: Res<Time>,
    mut timer: ResMut<GreetTimer>,
    mut query: Query<(&Person, &Name)>
) {
    timer.0.tick(time.delta_seconds);
    if timer.0.finished {
        for (_person, name) in &mut query.iter() {
            println!("{:?}", name);
            println!("hello {}", name.0);
        }
    }

}

pub struct HelloPlugin;

impl Plugin for HelloPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
            .add_resource(GreetTimer(Timer::from_seconds(2.0, true)))
            .add_startup_system(add_people.system())
            .add_system(greet_people.system());
    }
}

fn main() {
    App::build()
        .add_default_plugins()
        .add_plugin(HelloPlugin)
        .run();
}

