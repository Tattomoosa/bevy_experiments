use bevy::prelude::*;

mod input;

pub struct MainCamera {}

fn main() {
  App::build()
    .add_default_plugins()
    .add_plugin(input::InputPlugin {})
    .add_startup_system(setup.system())
    .run();
}

fn setup(mut commands: Commands) {
    commands
        .spawn(Camera2dComponents::default())
        .with(MainCamera {})
        ;
}
