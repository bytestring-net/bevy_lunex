use crate::*;
use bevy::{utils::HashMap, window::PrimaryWindow};


// #===================#
// #=== CURSOR TYPE ===#

#[derive(Component, Default)]
pub struct Cursor2d {
    /// Indicates which cursor is being requested.
    cursor_request: CursorIcon,
    /// Indicates the priority of the requested cursor.
    cursor_request_priority: f32,
    /// Map which cursor has which atlas index and offset
    cursor_atlas_map: HashMap<CursorIcon, (usize, Vec2)>,
    /// A toggle if this cursor should replace the native cursor
    native_cursor: bool,
    /// A toggle if the cursor should be hidden
    pub hidden: bool,
}
impl Cursor2d {
    /// Creates new default Cursor2d.
    pub fn new() -> Cursor2d {
        Cursor2d {
            cursor_request: CursorIcon::Default,
            cursor_request_priority: 0.0,
            cursor_atlas_map: HashMap::new(),
            native_cursor: true,
            hidden: false,
        }
    }
    /// A toggle if this cursor should be native
    pub fn native_cursor(mut self, enable: bool) -> Self {
        self.native_cursor = enable;
        self
    }
    /// A method to request a new cursor icon. Works only if priority is higher than already set priority this tick.
    pub fn request_cursor(&mut self, request: CursorIcon, priority: f32) {
        if priority > self.cursor_request_priority {
            self.cursor_request = request;
            self.cursor_request_priority = priority;
        }
    }
    /// Adds a new index and offset to the cursor.
    pub fn register_cursor(mut self, icon: CursorIcon, index: usize, offset: impl Into<Vec2>) -> Self {
        self.cursor_atlas_map.insert(icon, (index, offset.into()));
        self
    }
}


fn cursor_update( mut windows: Query<&mut Window, With<PrimaryWindow>>, mut query: Query<(&Cursor2d, &mut Transform, &mut Visibility)>) {
    if let Ok(mut window) = windows.get_single_mut() {
        for (cursor, mut transform, mut visibility) in &mut query {

            window.cursor.visible = if cursor.native_cursor { !cursor.hidden } else { false };
            if window.cursor.visible { window.cursor.icon = cursor.cursor_request; }

            match window.cursor_position() {
                Some(position) => {

                    let sprite_offset = cursor.cursor_atlas_map.get(&cursor.cursor_request).unwrap_or(&(0, Vec2::ZERO)).1;

                    transform.translation.x = position.x - window.width()*0.5 - sprite_offset.x * transform.scale.x;
                    transform.translation.y = -(position.y - window.height()*0.5 - sprite_offset.y * transform.scale.y);
                    *visibility = if cursor.hidden || cursor.native_cursor { Visibility::Hidden } else { Visibility::Visible };
                }
                None => {
                    *visibility = Visibility::Hidden;
                }
            }
        }
    }
}
fn cursor_preupdate(mut query: Query<&mut Cursor2d>) {
    for mut cursor in &mut query {
        cursor.cursor_request = CursorIcon::Default;
        cursor.cursor_request_priority = 0.0;
    }
}
fn cursor_update_texture(mut query: Query<(&Cursor2d, &mut TextureAtlas)>) {
    for (cursor, mut atlas) in &mut query {
        atlas.index = cursor.cursor_atlas_map.get(&cursor.cursor_request).unwrap_or(&(0, Vec2::ZERO)).0;
    }
}


/// Requests cursor icon on hover
#[derive(Component, Debug, Clone, PartialEq)]
pub struct OnHoverSetCursor {
    /// Cursor type to request on hover
    pub cursor: CursorIcon,
}
impl OnHoverSetCursor {
    /// Creates new struct
    pub fn new(cursor: CursorIcon) -> Self {
        OnHoverSetCursor {
            cursor
        }
    }
}
fn on_hover_set_cursor(query: Query<(&UiAnimation<Hover>, &OnHoverSetCursor)>, mut cursor: Query<&mut Cursor2d>) {
    for (control, hover_cursor) in &query {
        if control.is_forward() {
            let mut cursor = cursor.single_mut();
            cursor.request_cursor(hover_cursor.cursor, 1.0);
        }
    }
}


// #==============#
// #=== PLUGIN ===#

pub struct CursorPlugin;
impl Plugin for CursorPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(PreUpdate,  cursor_preupdate)
            .add_systems(PostUpdate, cursor_update)
            .add_systems(PostUpdate, cursor_update_texture)
            .add_systems(Update, on_hover_set_cursor);
    }
}