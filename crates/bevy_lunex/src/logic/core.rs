use crate::*;


// #==============#
// #=== EVENTS ===#

/// This is an event you can listen to which broadcasts the entity the pointer clicked on.
/// [`UiClickEmitter`] is the component creating these events.
#[derive(Event, Debug, Clone, PartialEq, Eq)]
pub struct UiClickEvent {
    /// The targetted entity that was clicked on
    pub target: Entity,
}

/// This is an event you can listen to which broadcasts the entity that changed its value.
/// This event is for example created when you change values in text-input field, spinbox,
/// radio button, etc.
#[derive(Event, Debug, Clone, PartialEq, Eq)]
pub struct UiChangeEvent {
    /// The targetted entity that changed its value
    pub target: Entity,
    /// The new value
    pub value: String,
}


// #=================#
// #=== LISTENERS ===#

/// When clicked on this entity, it will create [`UiClickEvent`] event for the specified entity.
/// This component is commonly used in abstraction, where you want to listen to pointer events
/// from another entity that is not the parent and send that data over.
#[derive(Component, Debug, Clone, PartialEq, Eq)]
pub struct UiClickEmitter {
    pub trigger: Option<Entity>,
}
impl UiClickEmitter {
    /// The entity will create the event for itself and not other entities.
    pub const SELF: UiClickEmitter = UiClickEmitter { trigger: None };
    /// Specify the entity you want to create events for.
    pub fn new(entity: Entity) -> Self {
        UiClickEmitter {
            trigger: Some(entity)
        }
    }
}

/// System that triggers when a pointer clicks a node and emmits an event
fn ui_click_listener_system(mut events: EventReader<Pointer<Down>>, mut write: EventWriter<UiClickEvent>, query: Query<(&UiClickEmitter, Entity)>) {
    for event in events.read() {
        if let Ok((emitter, entity)) = query.get(event.target) {
            write.send(UiClickEvent {
                target: if let Some(e) = emitter.trigger { e } else { entity },
            });
        }
    }
}


// #====================#
// #=== HOVER PLUGIN ===#

/// Plugin adding all our logic
pub struct CorePlugin;
impl Plugin for CorePlugin {
    fn build(&self, app: &mut App) {
        app
            // Add our events

            .add_event::<UiClickEvent>()
            .add_systems(Update, ui_click_listener_system.run_if(on_event::<Pointer<Down>>()))
            
            .add_event::<UiChangeEvent>()
            ;
    }
}