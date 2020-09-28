use bevy::prelude::*;
use bevy::render::camera::Camera;

// use crate::MainCamera;

#[derive(Default)]
struct Cursor {
    offset: Vec2,
}

fn screen_to_world(
    screen_pos: Vec2,
    camera_transform: &Transform,
    window: &Window
) -> Vec3 {
    let size = Vec2::new(window.width as f32, window.height as f32);
    let screen_position = screen_pos - size / 2.0;
    screen_position.extend(0.0) + camera_transform.translation()
}

fn update_cursor(
    mut state: ResMut<InputState>,
    cursor_event: Res<Events<CursorMoved>>,
    windows: Res<Windows>,
    mut mouse_query: Query<(&mut Transform, &Cursor)>,
    camera_transform_query: Query<&Transform>,
    mut camera_query: Query<(Entity, &Camera)>
) {
    if let Some(camera_entity) = state.main_camera {
        let camera_transform = camera_transform_query.get::<Transform>(camera_entity).unwrap();
        if let Some(ev) = state.cursor.iter(&cursor_event).last() {
            let window = windows.get(ev.id).unwrap();
            let cursor_position = screen_to_world(ev.position, &camera_transform, window);
            for (mut transform, cursor) in &mut mouse_query.iter() {
                transform.set_translation(cursor_position + cursor.offset.extend(0.0));
            }
        }
    } else {
        println!("cursor - finding camera...");
        for (camera_entity, _camera) in &mut camera_query.iter() {
            println!("{:?}", camera_entity);
            state.main_camera = Some(camera_entity)
        }
    }
}

pub fn setup_mouse(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // sprite
    let texture_handle = asset_server.load("assets/ui/mouse_cursors.png").unwrap();
    let mouse_sprite = SpriteComponents {
                material: materials.add(texture_handle.into()),
                ..Default::default()
            };
    commands
        // for some reason the spritecomponents need to be spawned alone
        // then others can be added with ".with"
        .spawn(mouse_sprite)
        .with(Cursor {
            offset: Vec2::new(10.0, 0.0)
        })
        ;
}

#[derive(Default)]
struct InputState {
    main_camera: Option<Entity>,
    cursor: EventReader<CursorMoved>
}

pub struct InputPlugin {}

impl Plugin for InputPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
            .add_resource(InputState::default())
            .add_startup_system(setup_mouse.system())
            // .add_system(update_main_camera.system())
            .add_system(update_cursor.system())
            ;
    }
}
