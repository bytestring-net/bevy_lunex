use bevy::prelude::*;
#[cfg(feature = "debug")]
use colored::Colorize;
use lunex_engine::UiLayout;

use crate::Cursor2d;


// #==============#
// #=== EVENTS ===#

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
            .add_event::<HideCursor2d>()
            .add_systems(Update, apply_event_hide_cursor_2d.run_if(on_event::<HideCursor2d>()))

            .add_event::<SetUiLayout>()
            .add_systems(Update, apply_event_set_ui_layout.run_if(on_event::<SetUiLayout>()))

            .add_event::<SetColor>()
            .add_systems(Update, apply_event_set_color.run_if(on_event::<SetColor>()));
    }
}