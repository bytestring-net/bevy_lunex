# Interactivity

Lunex implements its interactivity on top of `Bevy_mod_picking`. If you need more control over interactions, consider researching this crate.

Interactivity is achieved by utilizing Events and Systems. Lunex provides several components to simplify the process. First, ensure your entity is pickable by adding `PickableBundle` for entities with sprites or meshes, and `UiZoneBundle` for entities without.

To block picking, set `Pickable::IGNORE` on non-UI entities.
```rust
// Make it non-obsructable for hit checking (mouse detection)
Pickable::IGNORE,
```

To check for mouse clicks, listen to `UiClickEvent` in your systems using the following code:
```rust
fn button_click_system(mut events: EventReader<UiClickEvent>, query: Query<&CustomButton>) {
    for event in events.read() {
        if let Ok(button) = query.get(event.target) {
            info!("Pressed button: {}", button.text);
        }
    }
}
```

Note that `UiClickEvent` is not emitted automatically; you need to add the component to emit this event when `Pointer<Down>` is triggered if you decide to make your own components.
```rust
// If we click on this node, it will emmit UiClick event on itself
UiClickEmitter::SELF,

// If we click on this node, it will emmit UiClick event from specified entity
UiClickEmitter::new(entity),
```

This component is really useful when creating complex components. You want `UiClickEvent` to be emmited from the top entity in a component, so users can listen to them. This component allows you to do exactly that.


Another components that might prove useful are:
```rust
// If it detects UiClick event for this entity it will despawn the specified entity, great for despawning routes
OnUiClickDespawn::new(entity),

// If it detects UiClick event for this entity it will run the closure, great for spawning routes
OnUiClickCommands::new(|commands| { commands.spawn(MyRoute); })
```