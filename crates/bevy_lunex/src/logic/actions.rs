use bevy::window::{EnabledButtons, PresentMode, PrimaryWindow, WindowMode};

use crate::*;


// #=====================#
// #=== ACTION EVENTS ===#
// These events behave like [`AppExit`] event.
// When you call them something will happen.

// #=== WINDOW ===#

/// This event will change the primary window presentation mode (VSYNC)
#[derive(Event, Debug, Clone, PartialEq)]
pub struct SetWindowPresentMode (pub PresentMode);
fn set_window_present_mode_action(mut events: EventReader<SetWindowPresentMode>, mut query: Query<&mut Window, With<PrimaryWindow>>) {
    for event in events.read() {
        if let Ok(window) = &mut query.get_single_mut() {
            #[cfg(feature = "debug")]
            info!("{} - Changed present mode to: {:?}", "ACTION".red().bold(), event.0);
            window.present_mode = event.0;
        }
    }
}

/// This event will change the primary window mode (FULLSCREEN)
#[derive(Event, Debug, Clone, PartialEq)]
pub struct SetWindowMode (pub WindowMode);
fn set_window_mode_action(mut events: EventReader<SetWindowMode>, mut query: Query<&mut Window, With<PrimaryWindow>>) {
    for event in events.read() {
        if let Ok(window) = &mut query.get_single_mut() {
            #[cfg(feature = "debug")]
            info!("{} - Changed window mode to: {:?}", "ACTION".red().bold(), event.0);
            window.mode = event.0;
        }
    }
}

/// This event will change the primary window position
#[derive(Event, Debug, Clone, PartialEq)]
pub struct SetWindowPosition (pub WindowPosition);
fn set_window_position_action(mut events: EventReader<SetWindowPosition>, mut query: Query<&mut Window, With<PrimaryWindow>>) {
    for event in events.read() {
        if let Ok(window) = &mut query.get_single_mut() {
            #[cfg(feature = "debug")]
            info!("{} - Changed window position to: {:?}", "ACTION".red().bold(), event.0);
            window.position = event.0;
        }
    }
}

/// This event will change the primary window title
#[derive(Event, Debug, Clone, PartialEq)]
pub struct SetWindowTitle (pub String);
fn set_window_title_action(mut events: EventReader<SetWindowTitle>, mut query: Query<&mut Window, With<PrimaryWindow>>) {
    for event in events.read() {
        if let Ok(window) = &mut query.get_single_mut() {
            #[cfg(feature = "debug")]
            info!("{} - Changed window title to: {:?}", "ACTION".red().bold(), event.0);
            window.title = event.0.clone();
        }
    }
}

/// This event will change the primary window size
#[derive(Event, Debug, Clone, PartialEq)]
pub struct SetWindowResolution (pub Vec2);
fn set_window_resolution_action(mut events: EventReader<SetWindowResolution>, mut query: Query<&mut Window, With<PrimaryWindow>>) {
    for event in events.read() {
        if let Ok(window) = &mut query.get_single_mut() {
            #[cfg(feature = "debug")]
            info!("{} - Changed window resolution to: {:?}", "ACTION".red().bold(), event.0);
            window.resolution.set(event.0.x, event.0.y);
        }
    }
}

/// This event will change the primary window resize contstrains
#[derive(Event, Debug, Clone, PartialEq)]
pub struct SetWindowResizeConstrains (pub WindowResizeConstraints);
fn set_window_resize_constrains_action(mut events: EventReader<SetWindowResizeConstrains>, mut query: Query<&mut Window, With<PrimaryWindow>>) {
    for event in events.read() {
        if let Ok(window) = &mut query.get_single_mut() {
            #[cfg(feature = "debug")]
            info!("{} - Changed window resize constrains to: {:?}", "ACTION".red().bold(), event.0);
            window.resize_constraints = event.0;
        }
    }
}

/// This event will change if primary window is resizable
#[derive(Event, Debug, Clone, PartialEq)]
pub struct SetWindowResizable (pub bool);
fn set_window_resizable_action(mut events: EventReader<SetWindowResizable>, mut query: Query<&mut Window, With<PrimaryWindow>>) {
    for event in events.read() {
        if let Ok(window) = &mut query.get_single_mut() {
            #[cfg(feature = "debug")]
            info!("{} - Changed window resizable to: {:?}", "ACTION".red().bold(), event.0);
            window.resizable = event.0;
        }
    }
}

/// This event will change the enabled buttons for the primary window
#[derive(Event, Debug, Clone, PartialEq)]
pub struct SetWindowEnabledButtons (pub EnabledButtons);
fn set_window_enabled_buttons_action(mut events: EventReader<SetWindowEnabledButtons>, mut query: Query<&mut Window, With<PrimaryWindow>>) {
    for event in events.read() {
        if let Ok(window) = &mut query.get_single_mut() {
            #[cfg(feature = "debug")]
            info!("{} - Changed window buttons to: {:?}", "ACTION".red().bold(), event.0);
            window.enabled_buttons = event.0;
        }
    }
}

/// This event will change if primary window decorations are available
#[derive(Event, Debug, Clone, PartialEq)]
pub struct SetWindowDecorations (pub bool);
fn set_window_decorations_action(mut events: EventReader<SetWindowDecorations>, mut query: Query<&mut Window, With<PrimaryWindow>>) {
    for event in events.read() {
        if let Ok(window) = &mut query.get_single_mut() {
            #[cfg(feature = "debug")]
            info!("{} - Changed window decorations to: {:?}", "ACTION".red().bold(), event.0);
            window.decorations = event.0;
        }
    }
}

/// This event will focus OS on the primary window
#[derive(Event, Debug, Clone, PartialEq)]
pub struct SetWindowFocus (pub bool);
fn set_window_focus_action(mut events: EventReader<SetWindowFocus>, mut query: Query<&mut Window, With<PrimaryWindow>>) {
    for event in events.read() {
        if let Ok(window) = &mut query.get_single_mut() {
            #[cfg(feature = "debug")]
            info!("{} - Changed window focus to: {:?}", "ACTION".red().bold(), event.0);
            window.focused = event.0;
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

pub struct ActionsPlugin;
impl Plugin for ActionsPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<SetWindowPresentMode>()
            .add_systems(Update, set_window_present_mode_action.run_if(on_event::<SetWindowPresentMode>()))

            .add_event::<SetWindowMode>()
            .add_systems(Update, set_window_mode_action.run_if(on_event::<SetWindowMode>()))

            .add_event::<SetWindowPosition>()
            .add_systems(Update, set_window_position_action.run_if(on_event::<SetWindowPosition>()))

            .add_event::<SetWindowTitle>()
            .add_systems(Update, set_window_title_action.run_if(on_event::<SetWindowTitle>()))

            .add_event::<SetWindowResolution>()
            .add_systems(Update, set_window_resolution_action.run_if(on_event::<SetWindowResolution>()))

            .add_event::<SetWindowResizeConstrains>()
            .add_systems(Update, set_window_resize_constrains_action.run_if(on_event::<SetWindowResizeConstrains>()))

            .add_event::<SetWindowResizable>()
            .add_systems(Update, set_window_resizable_action.run_if(on_event::<SetWindowResizable>()))

            .add_event::<SetWindowEnabledButtons>()
            .add_systems(Update, set_window_enabled_buttons_action.run_if(on_event::<SetWindowEnabledButtons>()))

            .add_event::<SetWindowDecorations>()
            .add_systems(Update, set_window_decorations_action.run_if(on_event::<SetWindowDecorations>()))

            .add_event::<SetWindowFocus>()
            .add_systems(Update, set_window_focus_action.run_if(on_event::<SetWindowFocus>()))








            .add_event::<HideCursor2d>()
            .add_systems(Update, apply_event_hide_cursor_2d.run_if(on_event::<HideCursor2d>()))

            .add_event::<SetUiLayout>()
            .add_systems(Update, apply_event_set_ui_layout.run_if(on_event::<SetUiLayout>()))

            .add_event::<SetColor>()
            .add_systems(Update, apply_event_set_color.run_if(on_event::<SetColor>()));
    }
}