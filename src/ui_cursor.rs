#![allow(dead_code)]
#![allow(unused_variables)]

use bevy::prelude::*;


//===========================================================================

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
    pub fn position_world (&self) -> &Vec2 {
        &self.cursor_world
    }
    pub fn position_screen (&self) -> &Vec2 {
        &self.cursor_screen
    }
}

pub fn cursor_update(mut windows: Query<&mut Window>, mut query: Query<(&mut Cursor, &mut Transform)>) {
    for (mut cursorinfo, mut transform) in &mut query {
        let mut window = windows.get_single_mut().unwrap();

        match window.cursor_position() {
            Some (cursor) => {
                window.cursor.visible = false;

                let offset_x = window.resolution.width()/2. + cursorinfo.offset*transform.scale.x;
                let offset_y = window.resolution.height()/2. - cursorinfo.offset*transform.scale.y;

                cursorinfo.cursor_screen = Vec2 {x: cursor.x, y: cursor.y};
                cursorinfo.cursor_world = Vec2 {x: cursor.x - offset_x, y: cursor.y - offset_y};

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