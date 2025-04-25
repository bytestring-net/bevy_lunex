use crate::*;
use bevy::{input::{gamepad::GamepadButtonChangedEvent, mouse::MouseButtonInput, ButtonState}, picking::{pointer::{Location, PointerAction, PointerId, PointerInput, PointerLocation}, PickSet}, render::camera::{NormalizedRenderTarget, RenderTarget}, platform::collections::HashMap, window::{PrimaryWindow, SystemCursorIcon, WindowRef}, winit::cursor::CursorIcon};

// Exported prelude
pub mod prelude {
    // All standard exports
    pub use super::{
        SoftwareCursor,
        GamepadCursor,
        GamepadCursorMode,
        OnHoverSetCursor,
    };

    // Export stuff from other crates
    pub use bevy::window::SystemCursorIcon;
}

// #=========================#
// #=== CURSOR ICON QUEUE ===#

#[derive(Resource, Reflect, Clone, PartialEq, Debug, Default)]
pub struct CursorIconQueue {
    pointers: HashMap<PointerId, CursorQueueData>
}
impl CursorIconQueue {
    /// A method to request a new cursor icon. Works only if priority is higher than already set priority this tick.
    pub fn request_cursor(&mut self, pointer: PointerId, window: Option<Entity>, requestee: Entity, request: SystemCursorIcon, priority: usize) {
        if let Some(data) = self.pointers.get_mut(&pointer) {
            data.window = window;
            data.queue.insert(requestee, (request, priority));
        } else {
            let mut queue = HashMap::new();
            queue.insert(requestee, (request, priority));
            self.pointers.insert(pointer, CursorQueueData { window, queue, top_priority: 0, top_request: Default::default() });
        }
    }
    /// A method to cancel existing cursor in the queue stack
    pub fn cancel_cursor(&mut self, pointer: PointerId, requestee: &Entity) {
        if let Some(data) = self.pointers.get_mut(&pointer) {
            data.queue.remove(requestee);
        }
    }
}

#[derive(Reflect, Clone, PartialEq, Debug)]
struct CursorQueueData {
    window: Option<Entity>,
    top_priority: usize,
    top_request: SystemCursorIcon,
    queue: HashMap<Entity, (SystemCursorIcon, usize)>
}

/// This system will apply cursor changes to the windows it has in the resource.
fn system_cursor_icon_queue_apply(
    mut queue: ResMut<CursorIconQueue>,
    mut windows: Query<Option<&mut CursorIcon>, With<Window>>,
    mut commands: Commands,
) {
    if !queue.is_changed() { return; }
    for (_, data) in &mut queue.pointers {

        let mut top_priority = 0;
        let mut top_request = SystemCursorIcon::Default;

        // Look for highest priority to use
        for (_, (icon, priority)) in &data.queue {
            if *priority > top_priority {
                top_priority = *priority;
                top_request = *icon;
            }
        }

        data.top_priority = top_priority;
        data.top_request = top_request;

        if let Some(window) = data.window {
            if let Ok(window_cursor_option) = windows.get_mut(window) {

                // Apply the cursor icon somehow
                if let Some(mut window_cursor) = window_cursor_option {
                    #[allow(clippy::single_match)]
                    match window_cursor.as_mut() {
                        CursorIcon::System(previous) => {
                            if *previous != data.top_request {
                                *previous = data.top_request;
                            }
                        },
                        _ => {},
                    }

                } else {
                    commands.entity(window).insert(CursorIcon::System(data.top_request));
                }
            }
        }
    }
}

/// This system will cleanup the queue if any invalid data is found.
fn system_cursor_icon_queue_purge(
    mut queue: ResMut<CursorIconQueue>,
    mut windows: Query<&Window>,
    entities: Query<Entity>,
) {
    let mut to_remove = Vec::new();
    for (pointer, data) in &mut queue.pointers {

        // Remove invalid pointers
        if let Some(window) = data.window {
            if windows.get_mut(window).is_err() {
                to_remove.push(*pointer);
            }
        }

        // Remove despawned entities
        let mut to_remove = Vec::new();
        for (entity, _) in &data.queue {
            if entities.get(*entity).is_err() {
                to_remove.push(*entity);
            }
        }

        // Cleanup
        for entity in to_remove {
            data.queue.remove(&entity);
        }
    }

    // Cleanup
    for pointer in to_remove {
        queue.pointers.remove(&pointer);
    }
}


// #========================#
// #=== CURSOR ADDITIONS ===#

/// Requests cursor icon on hover
#[derive(Component, Reflect, Clone, PartialEq, Debug)]
#[require(Pickable::default())]
pub struct OnHoverSetCursor {
    /// SoftwareCursor type to request on hover
    pub cursor: SystemCursorIcon,
}
impl OnHoverSetCursor {
    /// Creates new struct
    pub fn new(cursor: SystemCursorIcon) -> Self {
        OnHoverSetCursor {
            cursor,
        }
    }
}

fn observer_cursor_request_cursor_icon(mut trigger: Trigger<Pointer<Over>>, mut pointers: Query<(&PointerId, &PointerLocation, Has<GamepadCursor>)>, query: Query<&OnHoverSetCursor>, mut queue: ResMut<CursorIconQueue>) {
    // Find the pointer location that triggered this observer
    let id = trigger.pointer_id;
    for (pointer, location, is_gamepad) in pointers.iter_mut().filter(|(p_id, _, _)| id == **p_id) {

        // Check if the pointer is attached to a window
        if let Some(location) = &location.location {
            if let NormalizedRenderTarget::Window(window) = location.target {

                // Request a cursor change
                if let Ok(requestee) = query.get(trigger.target) {
                    trigger.propagate(false);
                    queue.request_cursor(*pointer, if is_gamepad { None } else { Some(window.entity()) }, trigger.target, requestee.cursor, 1);
                }
            }
        }
    }
}

fn observer_cursor_cancel_cursor_icon(mut trigger: Trigger<Pointer<Out>>, mut pointers: Query<(&PointerId, &PointerLocation)>, query: Query<&OnHoverSetCursor>, mut queue: ResMut<CursorIconQueue>) {
    // Find the pointer location that triggered this observer
    let id = trigger.pointer_id;
    for (pointer, location) in pointers.iter_mut().filter(|(p_id, _)| id == **p_id) {

        // Check if the pointer is attached to a window
        if let Some(location) = &location.location {
            if matches!(location.target, NormalizedRenderTarget::Window(_)) {

                // Cancel existing cursor icon request if applicable
                if query.get(trigger.target).is_ok() {
                    trigger.propagate(false);
                    queue.cancel_cursor(*pointer, &trigger.target);
                }
            }
        }
    }
}



// #=======================#
// #=== SOFTWARE CURSOR ===#

/// Component for creating software mouse.
#[derive(Component, Reflect, Clone, PartialEq, Debug, Default)]
#[require(PointerId, Pickable = Pickable::IGNORE)]
pub struct SoftwareCursor {
    /// Indicates which cursor is being requested.
    cursor_request: SystemCursorIcon,
    /// Indicates the priority of the requested cursor.
    cursor_request_priority: f32,
    /// Map which cursor has which atlas index and offset
    cursor_atlas_map: HashMap<SystemCursorIcon, (usize, Vec2)>,
    /// Location of the cursor (same as [`Transform`] without sprite offset).
    pub location: Vec2,
}
impl SoftwareCursor {
    /// Creates new default SoftwareCursor.
    pub fn new() -> SoftwareCursor {
        SoftwareCursor {
            cursor_request: SystemCursorIcon::Default,
            cursor_request_priority: 0.0,
            cursor_atlas_map: HashMap::new(),
            location: Vec2::ZERO,
        }
    }
    /// A method to request a new cursor icon. Works only if priority is higher than already set priority this tick.
    pub fn request_cursor(&mut self, request: SystemCursorIcon, priority: f32) {
        if priority > self.cursor_request_priority {
            self.cursor_request = request;
            self.cursor_request_priority = priority;
        }
    }
    /// This function binds the specific cursor icon to an image index that is used if the entity has texture atlas attached to it.
    pub fn set_index(mut self, icon: SystemCursorIcon, index: usize, offset: impl Into<Vec2>) -> Self {
        self.cursor_atlas_map.insert(icon, (index, offset.into()));
        self
    }
}

/// This will make the [`SoftwareCursor`] controllable by a gamepad.
#[derive(Component, Reflect, Clone, PartialEq, Debug)]
pub struct GamepadCursor {
    /// This struct defines how should the cursor movement behave.
    pub mode: GamepadCursorMode,
    /// SoftwareCursor speed scale
    pub speed: f32,
}
impl GamepadCursor {
    /// Creates a new instance.
    pub fn new() -> Self {
        Self::default()
    }
}
impl Default for GamepadCursor {
    fn default() -> Self {
        Self { mode: Default::default(), speed: 1.0 }
    }
}

/// This struct defines how should the cursor movement behave.
#[derive(Debug, Clone, Default, PartialEq, Reflect)]
pub enum GamepadCursorMode {
    /// SoftwareCursor will freely move on input.
    #[default]
    Free,
    /// Will try to snap to nearby nodes on input.
    /// # WORK IN PROGRESS
    Snap,
}

/// This component is used for SoftwareCursor-Gamepad relation.
/// - It is added to a Gamepad if he has a virtual cursor assigned.
/// - It is added to a SoftwareCursor if he is assigned to an existing gamepad.
#[derive(Component, Reflect, Clone, PartialEq, Debug)]
pub struct GamepadAttachedCursor(pub Entity);



// #========================#
// #=== CURSOR FUNCTIONS ===#

/// This system will hide the native cursor.
fn system_cursor_hide_native(
    mut windows: Query<&mut Window>,
    query: Query<(&PointerLocation, Has<GamepadCursor>), With<SoftwareCursor>>
) {
    for (pointer_location, is_gamepad) in &query {
        if let Some(location) = &pointer_location.location {
            if let NormalizedRenderTarget::Window(window) = location.target {
                if let Ok(mut window) = windows.get_mut(window.entity()) {
                    window.cursor_options.visible = is_gamepad;
                }
            }
        }
    }
}

/// This system will hide the native cursor.
fn system_cursor_software_change_icon(
    icons: Res<CursorIconQueue>,
    mut query: Query<(&PointerId, &SoftwareCursor, &mut Sprite)>
) {
    for (pointer_id, software_cursor, mut sprite) in &mut query {
        if let Some(atlas) = &mut sprite.texture_atlas {
            if let Some(icon_data) = icons.pointers.get(pointer_id) {
                atlas.index = software_cursor.cursor_atlas_map.get(&icon_data.top_request).unwrap_or(&(0, Vec2::ZERO)).0;
            }
        }
    }
}

/// This system will attach any free cursor to the first gamepad it can find.
fn system_cursor_gamepad_assign(
    mut commands: Commands,
    cursors: Query<(Entity, &SoftwareCursor, &GamepadCursor), Without<GamepadAttachedCursor>>,
    gamepads: Query<(Entity, &Gamepad), Without<GamepadAttachedCursor>>,
) {
    let mut gamepads = gamepads.iter();
    if let Some((cursor, _, _)) = cursors.iter().next() {
        if let Some((gamepad, _)) = gamepads.next() {
            commands.entity(cursor).insert(GamepadAttachedCursor(gamepad));
            commands.entity(gamepad).insert(GamepadAttachedCursor(cursor));
            info!("Gamepad {gamepad} bound to cursor {cursor}");
        }
    }
}



/// This system will move the gamepad cursor.
fn system_cursor_gamepad_move(
    time: Res<Time>,
    gamepads: Query<&Gamepad, With<GamepadAttachedCursor>>,
    mut cursors: Query<(&mut SoftwareCursor, &GamepadCursor, &GamepadAttachedCursor), Without<Gamepad>>,
) {
    for (mut cursor, gamepad_settings, attached_gamepad) in &mut cursors {
        if let Ok(gamepad) = gamepads.get(attached_gamepad.0) {

            // Get the gamepad input
            let mut input = Vec2::new(
                gamepad.get(GamepadAxis::LeftStickX).unwrap_or(0.0),
                gamepad.get(GamepadAxis::LeftStickY).unwrap_or(0.0),
            );

            // Clamp the deadzone as a vector
            if input.length_squared() < 0.1 { input *= 0.0; }

            // Compute the cursor position change
            let x = input.x * gamepad_settings.speed * time.delta_secs() * 500.0;
            let y = input.y * gamepad_settings.speed * time.delta_secs() * 500.0;

            // Move the cursor if it changed
            if x != 0.0 { cursor.location.x += x; }
            if y != 0.0 { cursor.location.y += y; }
        }
    }
}

/// This system will move the mouse cursor.
fn system_cursor_mouse_move(
    windows: Query<&Window, With<PrimaryWindow>>,
    cameras: Query<&Projection>,
    mut query: Query<(&mut SoftwareCursor, Option<&ChildOf>), Without<GamepadCursor>>
) {
    if let Ok(window) = windows.single() {
        for (mut cursor, parent_option) in &mut query {
            if let Some(position) = window.cursor_position() {
                // Get projection scale to account for zoomed cameras
                let scale = if let Some(parent) = parent_option {
                    if let Ok(Projection::Orthographic(projection)) = cameras.get(parent.parent()) { projection.scale } else { 1.0 }
                } else { 1.0 };

                // Compute the cursor position
                let x = (position.x - window.width()*0.5) * scale;
                let y = -((position.y - window.height()*0.5) * scale);

                // Move the cursor if it changed
                if x != cursor.location.x { cursor.location.x = (position.x - window.width()*0.5) * scale; }
                if y != cursor.location.y { cursor.location.y = -((position.y - window.height()*0.5) * scale); }
            }
        }
    }
}



/// This system will update the transform component to reflect the sprite offset.
fn system_cursor_update_tranform(
    mut query: Query<(&SoftwareCursor, &mut Transform)>
) {
    for (cursor, mut transform) in &mut query {
        let sprite_offset = cursor.cursor_atlas_map.get(&cursor.cursor_request).unwrap_or(&(0, Vec2::ZERO)).1;
        transform.translation.x = cursor.location.x - sprite_offset.x * transform.scale.x;
        transform.translation.y = cursor.location.y + sprite_offset.y * transform.scale.y;
    }
}

/// This system will move the virtual pointer location.
fn system_cursor_move_pointer(
    windows: Query<(Entity, &Window), With<PrimaryWindow>>,
    mut query: Query<(&mut PointerLocation, &SoftwareCursor)>,
) {
    if let Ok((win_entity, window)) = windows.single() {
        for (mut pointer, cursor) in query.iter_mut() {
            // Change the pointer location
            pointer.location = Some(Location {
                target: RenderTarget::Window(WindowRef::Primary).normalize(Some(win_entity)).unwrap(),
                position: Vec2 {
                    x: cursor.location.x + window.width()/2.0,
                    y: -cursor.location.y + window.height()/2.0,
                }.round(),
            });
        }
    }
}



/// This system will send out pointer move events if they changed position
fn system_cursor_send_move_events(
    mut cursor_last: Local<HashMap<PointerId, Vec2>>,
    pointers: Query<(&PointerId, &PointerLocation), With<SoftwareCursor>>,
    mut pointer_output: EventWriter<PointerInput>,
) {
    // Send mouse movement events
    for (pointer, location) in &pointers {
        if let Some(location) = &location.location {
            let last = cursor_last.get(pointer).unwrap_or(&Vec2::ZERO);
            if *last == location.position { continue; }

            pointer_output.write(PointerInput::new(
                *pointer,
                Location {
                    target: location.target.clone(),
                    position: location.position,
                },
                PointerAction::Move {
                    delta: location.position - *last,
                },
            ));
            cursor_last.insert(*pointer, location.position);
        }
    }
}

/// This system will send out mouse pick events
fn system_cursor_mouse_send_pick_events(
    pointers: Query<&PointerLocation, (With<SoftwareCursor>, Without<GamepadCursor>)>,
    mut mouse_inputs: EventReader<MouseButtonInput>,
    mut pointer_output: EventWriter<PointerInput>,
) {
    // Send mouse movement events
    for location in &pointers {
        if let Some(location) = &location.location {

            // Send mouse click events
            for input in mouse_inputs.read() {

                // Which state to change
                match input.state {
                    ButtonState::Pressed => {
                        // Send out the event
                        pointer_output.write(PointerInput::new(
                            PointerId::Mouse,
                            Location {
                                target: location.target.clone(),
                                position: location.position,
                            },
                            PointerAction::Press(match input.button {
                                MouseButton::Left => PointerButton::Primary,
                                MouseButton::Right => PointerButton::Secondary,
                                MouseButton::Middle => PointerButton::Middle,
                                MouseButton::Other(_) | MouseButton::Back | MouseButton::Forward => continue,
                            }),
                        ));
                    },
                    ButtonState::Released => {
                        // Send out the event
                        pointer_output.write(PointerInput::new(
                            PointerId::Mouse,
                            Location {
                                target: location.target.clone(),
                                position: location.position,
                            },
                            PointerAction::Release(match input.button {
                                MouseButton::Left => PointerButton::Primary,
                                MouseButton::Right => PointerButton::Secondary,
                                MouseButton::Middle => PointerButton::Middle,
                                MouseButton::Other(_) | MouseButton::Back | MouseButton::Forward => continue,
                            }),
                        ));
                    },
                };
            }
        }
    }
}

/// This system will send out gamepad pick events
fn system_cursor_gamepad_send_pick_events(
    pointers: Query<&PointerLocation, (With<SoftwareCursor>, With<GamepadCursor>)>,
    mut mouse_inputs: EventReader<GamepadButtonChangedEvent>,
    mut pointer_output: EventWriter<PointerInput>,
) {
    // Send mouse movement events
    for location in &pointers {
        if let Some(location) = &location.location {

            // Send mouse click events
            for input in mouse_inputs.read() {


                // Which state to change
                match input.state {
                    ButtonState::Pressed => {
                        // Send out the event
                        pointer_output.write(PointerInput::new(
                            PointerId::Mouse,
                            Location {
                                target: location.target.clone(),
                                position: location.position,
                            },
                            PointerAction::Press(match input.button {
                                GamepadButton::South => PointerButton::Primary,
                                GamepadButton::East => PointerButton::Secondary,
                                GamepadButton::West => PointerButton::Middle,
                                _ => continue,
                            }),
                        ));
                    },
                    ButtonState::Released => {
                        // Send out the event
                        pointer_output.write(PointerInput::new(
                            PointerId::Mouse,
                            Location {
                                target: location.target.clone(),
                                position: location.position,
                            },
                            PointerAction::Release(match input.button {
                                GamepadButton::South => PointerButton::Primary,
                                GamepadButton::East => PointerButton::Secondary,
                                GamepadButton::West => PointerButton::Middle,
                                _ => continue,
                            }),
                        ));
                    },
                };
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
            // Add SoftwareCursor Icon Queue resource to the app
            .insert_resource(CursorIconQueue::default())
            .add_systems(PostUpdate, (
                system_cursor_icon_queue_purge,
                system_cursor_icon_queue_apply,
            ))

            // OnHoverSetCursor observers
            .add_observer(observer_cursor_request_cursor_icon)
            .add_observer(observer_cursor_cancel_cursor_icon)


            // #=== SOFTWARE CURSOR ===#

            // Add systems for emulating picking events
            .add_systems(First, (
                system_cursor_send_move_events,
                system_cursor_mouse_send_pick_events,
                system_cursor_gamepad_send_pick_events,
                ApplyDeferred
            ).chain().in_set(PickSet::Input))

            // Add core systems
            .add_systems(PreUpdate, (
                system_cursor_gamepad_move,
                system_cursor_mouse_move,
                system_cursor_update_tranform,
                system_cursor_move_pointer,
            ).chain())

            // Other stuff
            .add_systems(Update, (
                system_cursor_hide_native,
                system_cursor_software_change_icon,
                system_cursor_gamepad_assign,
            ));
    }
}