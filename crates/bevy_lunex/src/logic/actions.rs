use bevy::window::{EnabledButtons, PresentMode, PrimaryWindow, WindowMode};

use crate::*;


// #=====================#
// #=== ACTION EVENTS ===#
// These events behave like [`AppExit`] event.
// When you call them something will happen.

// #=== WINDOW ===#

/// This event will change the presentation mode (VSYNC)
#[derive(Event, Debug, Clone, PartialEq)]
pub struct SetWindowPresentModeAction (pub PresentMode);

/// This event will change the window mode (FULLSCREEN)
#[derive(Event, Debug, Clone, PartialEq)]
pub struct SetWindowModeAction (pub WindowMode);

/// This event will change the window position
#[derive(Event, Debug, Clone, PartialEq)]
pub struct SetWindowPositionAction (pub WindowPosition);

/// This event will change the window title
#[derive(Event, Debug, Clone, PartialEq)]
pub struct SetWindowTitleAction (pub String);

/// This event will change the window size
#[derive(Event, Debug, Clone, PartialEq)]
pub struct SetWindowResolutionAction (pub Vec2);

/// This event will change the window resize contstrains
#[derive(Event, Debug, Clone, PartialEq)]
pub struct SetWindowResizeConstrainsAction (pub WindowResizeConstraints);

/// This event will change if window is resizable
#[derive(Event, Debug, Clone, PartialEq)]
pub struct SetWindowResizableAction (pub bool);

/// This event will change the enabled buttons for the window
#[derive(Event, Debug, Clone, PartialEq)]
pub struct SetWindowEnabledButtonsAction (pub EnabledButtons);

/// This event will change if window decorations are available
#[derive(Event, Debug, Clone, PartialEq)]
pub struct SetWindowDecorationsAction (pub bool);

/// This event will focus OS on the window
#[derive(Event, Debug, Clone, PartialEq)]
pub struct SetWindowFocusAction (pub bool);







fn set_window_present_mode_action(mut events: EventReader<SetWindowPresentModeAction>, mut query: Query<&mut Window, With<PrimaryWindow>>) {
    for event in events.read() {
        if let Ok(window) = &mut query.get_single_mut() {
            #[cfg(feature = "debug")]
            info!("{} - Changed present mode to: {:?}", "ACTION".red().bold(), event.0);
            window.present_mode = event.0;
        }
    }
}

fn set_window_mode_action(mut events: EventReader<SetWindowModeAction>, mut query: Query<&mut Window, With<PrimaryWindow>>) {
    for event in events.read() {
        if let Ok(window) = &mut query.get_single_mut() {
            #[cfg(feature = "debug")]
            info!("{} - Changed window mode to: {:?}", "ACTION".red().bold(), event.0);
            window.mode = event.0;
        }
    }
}

fn set_window_position_action(mut events: EventReader<SetWindowPositionAction>, mut query: Query<&mut Window, With<PrimaryWindow>>) {
    for event in events.read() {
        if let Ok(window) = &mut query.get_single_mut() {
            #[cfg(feature = "debug")]
            info!("{} - Changed window position to: {:?}", "ACTION".red().bold(), event.0);
            window.position = event.0;
        }
    }
}

fn set_window_title_action(mut events: EventReader<SetWindowTitleAction>, mut query: Query<&mut Window, With<PrimaryWindow>>) {
    for event in events.read() {
        if let Ok(window) = &mut query.get_single_mut() {
            #[cfg(feature = "debug")]
            info!("{} - Changed window title to: {:?}", "ACTION".red().bold(), event.0);
            window.title = event.0.clone();
        }
    }
}

fn set_window_resolution_action(mut events: EventReader<SetWindowResolutionAction>, mut query: Query<&mut Window, With<PrimaryWindow>>) {
    for event in events.read() {
        if let Ok(window) = &mut query.get_single_mut() {
            #[cfg(feature = "debug")]
            info!("{} - Changed window resolution to: {:?}", "ACTION".red().bold(), event.0);
            window.resolution.set(event.0.x, event.0.y);
        }
    }
}

fn set_window_resize_constrains_action(mut events: EventReader<SetWindowResizeConstrainsAction>, mut query: Query<&mut Window, With<PrimaryWindow>>) {
    for event in events.read() {
        if let Ok(window) = &mut query.get_single_mut() {
            #[cfg(feature = "debug")]
            info!("{} - Changed window resize constrains to: {:?}", "ACTION".red().bold(), event.0);
            window.resize_constraints = event.0;
        }
    }
}










/// This event will override layout of targetted entity
#[derive(Event, PartialEq, Clone, Copy)]
pub struct HideCursor2d (pub bool);
fn apply_event_hide_cursor_2d(mut events: EventReader<HideCursor2d>, mut query: Query<&mut Cursor2d>) {
    for event in events.read() {
        for mut cursor in &mut query {
            #[cfg(feature = "debug")]
            info!("{} - Set cursor to hidden: {}", "EVENT".purple().bold(), event.0);
            cursor.hidden = event.0;
        }
    }
}

/// This event will override layout of targetted entity
#[derive(Event, PartialEq, Clone, Copy)]
pub struct SetUiLayout {
    pub target: Entity,
    pub layout: UiLayout,
}
fn apply_event_set_ui_layout(mut events: EventReader<SetUiLayout>, mut query: Query<&mut UiLayout>) {
    for event in events.read() {
        if let Ok(mut layout) = query.get_mut(event.target) {
            if layout.clone() != event.layout{
                *layout = event.layout;
            }
        }
    }
}

/// This event will override sprite/text color of targetted entity
#[derive(Event, PartialEq, Clone, Copy)]
pub struct SetColor {
    pub target: Entity,
    pub color: Color,
}
fn apply_event_set_color(mut events: EventReader<SetColor>, mut query: Query<(Option<&mut Sprite>, Option<&mut Text>)>) {
    for event in events.read() {
        if let Ok((sprite_option, text_option)) = query.get_mut(event.target) {
            if let Some(mut sprite) = sprite_option {
                sprite.color = event.color;
            }
            if let Some(mut text) = text_option {
                for section in &mut text.sections {
                    section.style.color = event.color;
                }
            }
        }
    }
}


// #==============#
// #=== PLUGIN ===#

pub struct UiEventPlugin;
impl Plugin for UiEventPlugin {
    fn build(&self, app: &mut App) {
        app

            .add_event::<SetWindowPresentModeAction>()
            .add_systems(Update, set_window_present_mode_action.run_if(on_event::<SetWindowPresentModeAction>()))

            .add_event::<SetWindowModeAction>()
            .add_systems(Update, set_window_mode_action.run_if(on_event::<SetWindowModeAction>()))

            .add_event::<SetWindowPositionAction>()
            .add_systems(Update, set_window_position_action.run_if(on_event::<SetWindowPositionAction>()))

            .add_event::<SetWindowTitleAction>()
            .add_systems(Update, set_window_title_action.run_if(on_event::<SetWindowTitleAction>()))

            .add_event::<SetWindowResolutionAction>()
            .add_systems(Update, set_window_resolution_action.run_if(on_event::<SetWindowResolutionAction>()))

            .add_event::<SetWindowResizeConstrainsAction>()
            .add_systems(Update, set_window_resize_constrains_action.run_if(on_event::<SetWindowResizeConstrainsAction>()))










            .add_event::<HideCursor2d>()
            .add_systems(Update, apply_event_hide_cursor_2d.run_if(on_event::<HideCursor2d>()))

            .add_event::<SetUiLayout>()
            .add_systems(Update, apply_event_set_ui_layout.run_if(on_event::<SetUiLayout>()))

            .add_event::<SetColor>()
            .add_systems(Update, apply_event_set_color.run_if(on_event::<SetColor>()));
    }
}