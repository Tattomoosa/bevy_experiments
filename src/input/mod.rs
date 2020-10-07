use bevy::prelude::*;
use bevy::{
    render::camera::Camera,
    winit::WinitWindows,
    window::WindowId,
};

// use crate::MainCamera;

enum CursorType {
    Cursor,
    Pointer
}

// returns which frame represents which cursor
impl CursorType {
    fn value(&self) -> u32 {
        match *self {
            CursorType::Cursor => 0,
            CursorType::Pointer => 1,
        }
    }
    fn offset(&self) -> Vec2 {
        match *self {
            CursorType::Cursor => Vec2::new(15.0, -15.0),
            CursorType::Pointer => Vec2::new(7.0, -15.0),
        }
    }
}

struct Cursor {
    cursor_type: CursorType,
}

// TODO move into custom camera type
fn screen_to_world(
    screen_pos: Vec2,
    camera_transform: &Transform,
    window: &Window
) -> Vec3 {
    let size = Vec2::new(window.width as f32, window.height as f32);
    let screen_relative = (screen_pos - size / 2.0).extend(0.0) *
        camera_transform.scale();
    screen_relative + camera_transform.translation()
}

fn update_cursor(
    mut state: ResMut<InputState>,
    cursor_event: Res<Events<CursorMoved>>,
    windows: Res<Windows>,
    mut mouse_query: Query<(&mut Transform, &Cursor, &mut TextureAtlasSprite)>,
    camera_transform_query: Query<&Transform>,
    mut camera_query: Query<(Entity, &Camera)>
) {
    if let Some(camera_entity) = state.main_camera {
        let camera_transform = camera_transform_query.get::<Transform>(camera_entity).unwrap();
        if let Some(ev) = state.cursor_events.iter(&cursor_event).last() {
            let window = windows.get(ev.id).unwrap();
            // TODO do this somewhere else
            // window.set_cursor_visible(false);
            for (mut transform, cursor, mut sprite) in &mut mouse_query.iter() {
                let cursor_offset = cursor.cursor_type.offset();
                let cursor_position = screen_to_world(ev.position + cursor_offset, &camera_transform, window);
                transform.set_translation(cursor_position);
                sprite.index = cursor.cursor_type.value();
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

fn hide_os_cursor(
    windows: Res<WinitWindows>
) {
    let window = windows.get_window(WindowId::primary()).unwrap();
    window.set_cursor_visible(false);
}

pub fn setup_mouse(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut textures: ResMut<Assets<Texture>>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    // sprite
    // TODO does this need to be sync?
    let texture_handle = asset_server
        .load_sync(
            &mut textures,
            "assets/ui/mouse_cursors.png"
        ).unwrap();
    let texture = textures.get(&texture_handle).unwrap();
    let texture_atlas = TextureAtlas::from_grid(
        texture_handle,
        texture.size,
        2,
        1
        );
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    let mouse_sprite = SpriteSheetComponents {
        texture_atlas: texture_atlas_handle,
        ..Default::default()
    };
    commands
        // for some reason the spritecomponents need to be spawned alone
        // then others can be added with ".with"
        .spawn(mouse_sprite)
        .with(Cursor {
            cursor_type: CursorType::Pointer,
        })
    ;
}

#[derive(Default)]
struct InputState {
    main_camera: Option<Entity>,
    cursor_events: EventReader<CursorMoved>,
}

pub struct InputPlugin {}

impl Plugin for InputPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
            .add_resource(InputState::default())
            .add_startup_system(setup_mouse.system())
            .add_system(hide_os_cursor.system())
            .add_system(update_cursor.system())
            ;
    }
}
