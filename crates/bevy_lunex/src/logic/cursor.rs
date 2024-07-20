use crate::*;
use bevy::{input::{gamepad::GamepadButtonChangedEvent, mouse::MouseButtonInput, ButtonState}, render::camera::RenderTarget, utils::HashMap, window::{CursorGrabMode, PrimaryWindow, WindowRef}};
use picking_core::PickSet;
use pointer::{InputMove, InputPress, Location};

// #===================#
// #=== CURSOR TYPE ===#

/// Component for easy cursor control.
/// Read more about it in the [docs](https://bytestring-net.github.io/bevy_lunex/advanced/3_cursor.html)
#[derive(Component, Debug, Clone, PartialEq)]
pub struct Cursor2d {
    /// Indicates which cursor is being requested.
    cursor_request: CursorIcon,
    /// Indicates the priority of the requested cursor.
    cursor_request_priority: f32,
    /// Map which cursor has which atlas index and offset
    cursor_atlas_map: HashMap<CursorIcon, (usize, Vec2)>,
    /// Location of the cursor (same as [`Transform`] without sprite offset).
    pub location: Vec2,
    /// If the cursor is allowed to leave window. Does nothing is cursor is controlled by gamepad.
    pub confined: bool,
    /// A toggle if the cursor should be visible
    pub visible: bool,
}
impl Cursor2d {
    /// Creates new default Cursor2d.
    pub fn new() -> Cursor2d {
        Cursor2d {
            cursor_request: CursorIcon::Default,
            cursor_request_priority: 0.0,
            cursor_atlas_map: HashMap::new(),
            location: Vec2::ZERO,
            confined: false,
            visible: true,
        }
    }
    /// A method to request a new cursor icon. Works only if priority is higher than already set priority this tick.
    pub fn request_cursor(&mut self, request: CursorIcon, priority: f32) {
        if priority > self.cursor_request_priority {
            self.cursor_request = request;
            self.cursor_request_priority = priority;
        }
    }
    /// This function binds the specific cursor icon to an image index that is used if the entity has texture atlas attached to it.
    pub fn set_index(mut self, icon: CursorIcon, index: usize, offset: impl Into<Vec2>) -> Self {
        self.cursor_atlas_map.insert(icon, (index, offset.into()));
        self
    }
}
impl Default for Cursor2d {
    fn default() -> Self {
        Self {
            cursor_request: Default::default(),
            cursor_request_priority: Default::default(),
            cursor_atlas_map: Default::default(),
            location: Default::default(),
            confined: Default::default(),
            visible: true,
        }
    }
}

/// This will make the [`Cursor2d`] controllable by specific gamepad.
#[derive(Component, Debug, Clone, PartialEq)]
pub struct GamepadCursor {
    /// Gamepad index
    pub id: usize,
    /// This struct defines how should the cursor movement behave.
    pub mode: GamepadCursorMode,
    /// Cursor speed scale
    pub speed: f32,
}
impl GamepadCursor {
    /// Creates a new instance from gamepad id.
    pub fn new(id: usize) -> Self {
        Self { id, ..Default::default() }
    }
}
impl Default for GamepadCursor {
    fn default() -> Self {
        Self { id: 0, mode: Default::default(), speed: 1.0 }
    }
}


/// This struct defines how should the cursor movement behave.
#[derive(Debug, Clone, Default, PartialEq)]
pub enum GamepadCursorMode {
    /// Cursor will freely move on input.
    #[default]
    Free,
    // /// Will try to snap to nearby nodes on input.
    //Snap,
}


// #======================#
// #=== CURSOR BUNDLES ===#

/// Use this bundle to spawn native cursor
#[derive(Bundle)]
pub struct CursorBundle {
    /// Main cursor component
    pub cursor: Cursor2d,
    /// The virtual pointer that the cursor controls
    pub pointer: PointerBundle,
    /// Required for Cursor to exist
    pub spatial: SpatialBundle,
}
impl Default for CursorBundle {
    fn default() -> Self {
        Self {
            cursor: default(),
            pointer: PointerBundle::new(PointerId::Custom(pointer::Uuid::new_v4())),
            spatial: default(),
        }
    }
}

/// Use this bundle to spawn styled custom cursor
#[derive(Bundle)]
pub struct StyledCursorBundle {
    /// Main cursor component
    pub cursor: Cursor2d,
    /// The virtual pointer that the cursor controls
    pub pointer: PointerBundle,
    /// Sprite atlas for the cursor
    pub atlas: TextureAtlas,
    /// Sprite cursor
    pub sprite: SpriteBundle,
    /// Required to be [`Pickable::IGNORE`]
    pub pickable: Pickable,
}
impl Default for StyledCursorBundle {
    fn default() -> Self {
        Self {
            cursor: default(),
            pointer: PointerBundle::new(PointerId::Custom(pointer::Uuid::new_v4())),
            atlas: default(),
            sprite: default(),
            pickable: Pickable::IGNORE,
        }
    }
}


// #========================#
// #=== CURSOR FUNCTIONS ===#

/// This function controls the visibility of the cursor
fn cursor_set_visibility(
    mut windows: Query<&mut Window, With<PrimaryWindow>>,
    mut query: Query<(&Cursor2d, Option<&mut Visibility>, Has<GamepadCursor>, Has<Handle<Image>>)>
) {
    if let Ok(mut window) = windows.get_single_mut() {
        for (cursor, optional_visibility, has_gamepad, has_image) in &mut query {
            // If we have visibility then change it
            if let Some(mut visibility) = optional_visibility {
                *visibility = if cursor.visible { Visibility::Visible } else { Visibility::Hidden };
                if window.cursor_position().is_none() && !has_gamepad { *visibility = Visibility::Hidden }
            }

            // If it is not a gamepad
            if !has_gamepad {
                // Set native cursor to invisible if image is attached to the cursor
                window.cursor.visible = if has_image { false } else { cursor.visible };
            }
        }
    }
}

/// This function controls the native mouse cursor settings
fn cursor_change_native(
    mut windows: Query<&mut Window, With<PrimaryWindow>>,
    mut query: Query<&Cursor2d, Without<GamepadCursor>>
) {
    if let Ok(mut window) = windows.get_single_mut() {
        for cursor in &mut query {
            // Change native cursor
            if window.cursor.visible { window.cursor.icon = cursor.cursor_request; }

            // Change grab mode
            window.cursor.grab_mode = if cursor.confined { CursorGrabMode::Confined } else { CursorGrabMode::None }
        }
    }
}


/// This function controls the location of the cursor based on gamepad input
fn gamepad_move_cursor(
    axis: Res<Axis<GamepadAxis>>,
    time: Res<Time>,
    windows: Query<&Window, With<PrimaryWindow>>,
    mut query: Query<(&mut Cursor2d, &GamepadCursor)>,
) {
    if let Ok(window) = windows.get_single() {
        for (mut cursor, gamepad) in query.iter_mut() {
            // Pull axis values
            let x = axis.get(GamepadAxis { gamepad: Gamepad::new(gamepad.id), axis_type: GamepadAxisType::LeftStickX });
            let y = axis.get(GamepadAxis { gamepad: Gamepad::new(gamepad.id), axis_type: GamepadAxisType::LeftStickY });

            if let (Some(x), Some(y)) = (x, y) {
                // Move the cursor
                cursor.location.x += x * time.delta_seconds() * 500.0 * gamepad.speed;
                cursor.location.y += y * time.delta_seconds() * 500.0 * gamepad.speed;

                // Clamp the cursor within window
                let w = window.width()/2.0;
                let h = window.height()/2.0;
                cursor.location.x = cursor.location.x.clamp(-w, w);
                cursor.location.y = cursor.location.y.clamp(-h, h);
            }
        }
    }
}

/// This function controls the location of the cursor based on mouse input
fn mouse_move_cursor(
    windows: Query<&Window, With<PrimaryWindow>>,
    cameras: Query<&OrthographicProjection>,
    mut query: Query<(&mut Cursor2d, Option<&Parent>), Without<GamepadCursor>>
) {
    if let Ok(window) = windows.get_single() {
        for (mut cursor, parent_option) in &mut query {
            if let Some(position) = window.cursor_position() {
                // Get projection scale to account for zoomed cameras
                let scale = if let Some(parent) = parent_option {
                    if let Ok(projection) = cameras.get(**parent) { projection.scale } else { 1.0 }
                } else { 1.0 };
                

                // Move the cursor
                cursor.location.x = (position.x - window.width()*0.5) * scale;
                cursor.location.y = -((position.y - window.height()*0.5) * scale);
            }
        }
    }
}

/// This function updates the transform component with the modified location and sprite offsets
fn cursor_update_transform(
    mut query: Query<(&Cursor2d, &mut Transform)>
) {
    for (cursor, mut transform) in &mut query {
        let sprite_offset = cursor.cursor_atlas_map.get(&cursor.cursor_request).unwrap_or(&(0, Vec2::ZERO)).1;
        transform.translation.x = cursor.location.x - sprite_offset.x * transform.scale.x;
        transform.translation.y = cursor.location.y + sprite_offset.y * transform.scale.y;
    }
}

/// This function controls virtual pointer attached to the cursor
fn cursor_move_virtual_pointer(
    windows: Query<(Entity, &Window), With<PrimaryWindow>>,
    mut query: Query<(&mut PointerLocation, &Cursor2d)>,
) {
    if let Ok((win_entity, window)) = windows.get_single() {
        for (mut pointer, cursor) in query.iter_mut() {
            // Change the pointer location
            pointer.location = Some(pointer::Location {
                target: RenderTarget::Window(WindowRef::Primary).normalize(Some(win_entity)).unwrap(),
                position: Vec2 {
                    x: cursor.location.x + window.width()/2.0,
                    y: -cursor.location.y + window.height()/2.0,
                }.round(),
            });
        }
    }
}

/// This function sends mouse pointer events to be processed by the mod picking core plugin
fn cursor_mouse_pick_events(
    // Input
    mut mouse_inputs: EventReader<MouseButtonInput>,
    mut cursor_last: Local<HashMap<PointerId, Vec2>>,
    pointers: Query<(&PointerId, &PointerLocation), (With<Cursor2d>, Without<GamepadCursor>)>,
    // Output
    mut pointer_move: EventWriter<InputMove>,
    mut pointer_presses: EventWriter<InputPress>,
) {
    // Send mouse movement events
    for (pointer, location) in &pointers {
        if let Some(location) = &location.location {
            let last = cursor_last.get(pointer).unwrap_or(&Vec2::ZERO);
            if *last == location.position { continue; }

            pointer_move.send(InputMove::new(
                *pointer,
                Location {
                    target: location.target.clone(),
                    position: location.position,
                },
                location.position - *last,
            ));
            cursor_last.insert(*pointer, location.position);
        }
    }

    // Send mouse click events
    for input in mouse_inputs.read() {
        let button = match input.button {
            MouseButton::Left => PointerButton::Primary,
            MouseButton::Right => PointerButton::Secondary,
            MouseButton::Middle => PointerButton::Middle,
            MouseButton::Other(_) => continue,
            MouseButton::Back => continue,
            MouseButton::Forward => continue,
        };

        match input.state {
            ButtonState::Pressed => {
                for (pointer, _) in &pointers {
                    pointer_presses.send(InputPress::new_down(*pointer, button));
                }
            }
            ButtonState::Released => {
                for (pointer, _) in &pointers {
                    pointer_presses.send(InputPress::new_up(*pointer, button));
                }
            }
        }
    }
}

/// This function sends mouse pointer events to be processed by the mod picking core plugin
fn cursor_gamepad_pick_events(
    // Input
    mut gamepad_inputs: EventReader<GamepadButtonChangedEvent>,
    mut cursor_last: Local<HashMap<PointerId, Vec2>>,
    pointers: Query<(&PointerId, &PointerLocation, &GamepadCursor), With<Cursor2d>>,
    // Output
    mut pointer_move: EventWriter<InputMove>,
    mut pointer_presses: EventWriter<InputPress>,
) {
    // Send mouse movement events
    for (pointer, location, _) in &pointers {
        if let Some(location) = &location.location {
            let last = cursor_last.get(pointer).unwrap_or(&Vec2::ZERO);
            if *last == location.position { continue; }

            pointer_move.send(InputMove::new(
                *pointer,
                Location {
                    target: location.target.clone(),
                    position: location.position,
                },
                location.position - *last,
            ));
            cursor_last.insert(*pointer, location.position);
        }
    }

    // Send mouse click events
    for input in gamepad_inputs.read() {
        let button = match input.button_type {
            GamepadButtonType::South => PointerButton::Primary,
            GamepadButtonType::East => PointerButton::Secondary,
            GamepadButtonType::West => PointerButton::Middle,
            _ => continue,
        };

        match input.value {
            1.0 => {
                for (pointer, _, gamepad) in &pointers {
                    if gamepad.id != input.gamepad.id { continue; }
                    pointer_presses.send(InputPress::new_down(*pointer, button));
                }
            }
            0.0 => {
                for (pointer, _, gamepad) in &pointers {
                    if gamepad.id != input.gamepad.id { continue; }
                    pointer_presses.send(InputPress::new_up(*pointer, button));
                }
            },
            _ => {}
        }
    }
}


/// This function resets the requested cursor back to default every tick
fn cursor_reset_icon(
    mut query: Query<&mut Cursor2d>
) {
    for mut cursor in &mut query {
        cursor.cursor_request = CursorIcon::Default;
        cursor.cursor_request_priority = 0.0;
    }
}

/// This function updates the atlas index texture based on requested cursor icon
fn cursor_update_texture(
    mut query: Query<(&Cursor2d, &mut TextureAtlas)>
) {
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
fn on_hover_set_cursor(query: Query<(&UiAnimator<Hover>, &OnHoverSetCursor)>, mut cursor: Query<&mut Cursor2d>) {
    for (control, hover_cursor) in &query {
        if control.is_forward() {
            if let Ok(mut cursor) = cursor.get_single_mut(){
                cursor.request_cursor(hover_cursor.cursor, 1.0);
            }
        }
    }
}


// #==============#
// #=== PLUGIN ===#

pub struct CursorPlugin;
impl Plugin for CursorPlugin {
    fn build(&self, app: &mut App) {
        app

            // Add systems for mod picking event emitters
            .add_systems(First, (cursor_mouse_pick_events, cursor_gamepad_pick_events, apply_deferred).chain().in_set(PickSet::Input))

            // Add core systems 
            .add_systems(PreUpdate, cursor_reset_icon)
            .add_systems(PreUpdate, (gamepad_move_cursor, mouse_move_cursor, cursor_update_transform, cursor_move_virtual_pointer).chain())
            .add_systems(PostUpdate, cursor_set_visibility)
            .add_systems(PostUpdate, cursor_change_native)
            .add_systems(PostUpdate, cursor_update_texture)

            // Other stuff
            .add_systems(Update, on_hover_set_cursor);
    }
}