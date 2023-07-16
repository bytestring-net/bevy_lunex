use bevy::prelude::*;

// ===========================================================
// === CURSOR FUNCTIONALITY ===

#[derive(Component, Default)]
pub struct Cursor {
    depth: f32,
    offset: f32,
    cursor_world: Vec2,
    cursor_screen: Vec2,
}
impl Cursor {
    pub fn new (offset: f32) -> Cursor {
        Cursor {
            offset,
            ..Default::default()
        }
    }
    pub fn get_depth (&self) -> &f32 {
        &self.depth
    }
    pub fn get_depth_mut (&mut self) -> &mut f32 {
        &mut self.depth
    }
    pub fn position_world (&self) -> &Vec2 {
        &self.cursor_world
    }
    pub fn position_screen (&self) -> &Vec2 {
        &self.cursor_screen
    }
}

pub fn cursor_update(mut windows: Query<&mut Window>, cameras: Query<(&Camera, &Transform), Without<Cursor>>, mut query: Query<(&mut Cursor, &mut Transform), Without<Camera>>) {
    for (mut cursorinfo, mut transform) in &mut query {
        let mut window = windows.get_single_mut().unwrap();
        let (_, camera) = cameras.get_single().unwrap();

        match window.cursor_position() {
            Some (cursor) => {
                window.cursor.visible = false;

                let offset_x = window.resolution.width()/2.0 + cursorinfo.offset*transform.scale.x;
                let offset_y = window.resolution.height()/2.0 - cursorinfo.offset*transform.scale.y;

                cursorinfo.cursor_screen = Vec2 {x: cursor.x, y: cursor.y};
                cursorinfo.cursor_world = Vec2 {x: cursor.x - offset_x + camera.translation.x, y: window.resolution.height() - cursor.y - offset_y + camera.translation.y};
                cursorinfo.depth = 0.0;

                transform.translation.x = cursorinfo.cursor_world.x;
                transform.translation.y = cursorinfo.cursor_world.y;

            },
            None => {
                transform.translation.x = -window.resolution.width();
                transform.translation.y = -window.resolution.height();
            }
        }
    }
}