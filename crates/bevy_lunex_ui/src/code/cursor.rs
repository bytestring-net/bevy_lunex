use bevy::prelude::*;

// ===========================================================
// === CURSOR ===

#[derive(Component, Default)]
pub struct Cursor {
    default_cursor_index: usize,
    request_cursor_index: usize,
    sprite_offset: Vec<Vec2>,
    location_screen: Vec2,
    location_world: Vec2,
    os_cursor: bool,
}
impl Cursor {
    pub fn new() -> Cursor {
        Cursor {
            default_cursor_index: 0,
            request_cursor_index: 0,
            sprite_offset: Vec::new(),
            location_screen: Vec2::ZERO,
            location_world: Vec2::ZERO,
            os_cursor: true,
        }
    }
    pub fn with_os_cursor(mut self, enable: bool) -> Self {
        self.os_cursor = enable;
        self
    }
    pub fn add_sprite_offset(mut self, offset: Vec2) -> Self {
        self.sprite_offset.push(offset);
        self
    }
    pub fn location_screen(&self) -> &Vec2 {
        &self.location_screen
    }
    pub fn location_world(&self) -> &Vec2 {
        &self.location_world
    }
    pub fn with_default_cursor_index(mut self, index: usize) -> Self {
        self.default_cursor_index = usize::min(index, self.sprite_offset.len()-1);
        self
    }
    pub fn request_cursor_index(&mut self, index: usize) {
        self.request_cursor_index = usize::min(index, self.sprite_offset.len()-1)
    }
}

pub fn cursor_update(
    mut windows: Query<&mut Window>,
    cameras: Query<&Transform, (With<Camera>, Without<Cursor>)>,
    mut query: Query<(&mut Cursor, &mut Transform, &mut Visibility), Without<Camera>>,
) {
    for (mut cursor, mut transform, mut visibility) in &mut query {
        let mut window = windows.single_mut();
        let camera_transform = cameras.single();
        match window.cursor_position() {
            Some(win_cursor) => {
                window.cursor.visible = cursor.os_cursor;
                cursor.location_screen = win_cursor;
                let sprite_offset = if cursor.sprite_offset.len() != 0 {
                    cursor.sprite_offset[cursor.request_cursor_index]
                } else {
                    Vec2::ZERO
                };
                let offset_x = window.resolution.width() / 2.0;
                let offset_y = window.resolution.height() / 2.0;
                let world_x = camera_transform.translation.x + win_cursor.x;
                let world_y = camera_transform.translation.y + window.resolution.height() - win_cursor.y;
                cursor.location_world = Vec2::new(
                    world_x - offset_x,
                    world_y - offset_y
                );
                transform.translation.x = world_x - offset_x - sprite_offset.x * transform.scale.x;
                transform.translation.y = world_y - offset_y + sprite_offset.y * transform.scale.y;
                *visibility = Visibility::Visible;
            }
            None => {
                cursor.location_screen = Vec2::splat(-10000.0);
                cursor.location_world = Vec2::splat(-10000.0);
                *visibility = Visibility::Hidden;
            }
        }
    }
}

/// Set's the requested cursor index to be default
pub fn cursor_preupdate(mut query: Query<&mut Cursor>) {
    for mut cursor in &mut query {
        cursor.request_cursor_index = cursor.default_cursor_index;
    }
}

/// Applies requested cursor index as sprite index
pub fn cursor_update_texture(mut query: Query<(&Cursor, &mut TextureAtlasSprite)>) {
    for (cursor, mut sprite) in &mut query {
        sprite.index = cursor.request_cursor_index;
    }
}