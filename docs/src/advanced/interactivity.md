# Interactivity

Lunex implements its interactivity on top of `Bevy_mod_picking`. If you need more control over interactions, consider researching this crate.

This crate allows us to detect mouse clicks, hovers, drags, etc.

### Requirements

* Please note that `Cursor2d` **MUST** be spawned for any picking to work.

* `UiLunexPickingPlugin` must be added (Part of `UiDefaultPlugins` but not part of `UiMinimalPlugins`)

* `DefaultPickingPlugins.build().disable::<InputPlugin>()` version of picking plugins must also be added (Part of `UiDefaultPlugins` but not part of `UiMinimalPlugins`)

### Getting started

Interactivity is achieved by utilizing Events and Systems. Lunex provides several components to simplify the process. First, ensure your entity is pickable by adding `PickableBundle` for entities with sprites or meshes, and `UiZoneBundle` for entities without sprite or meshes.

Pay attention to **DEPTH** of entities (Not just UI entities), because if they overlap your pickable node, they will block all picking events. To avoid that, you can add `Pickable::IGNORE` component to any entity that might overlap your node.
```rust
// Make it non-obsructable for hit checking (mouse detection)
Pickable::IGNORE,
```

To check for mouse clicks, listen to `UiClickEvent` in your systems using the following code:
```rust
// Here we can listen to UiClick events that hold entity ID, then retrieve that entity from our query
fn button_click_system(mut events: EventReader<UiClickEvent>, query: Query<&CustomButton>) {
    // Iterate over all events
    for event in events.read() {
        // Get our entity
        if let Ok(button) = query.get(event.target) {
            // Process our button click
            info!("Pressed button: {}", button.text);
        }
    }
}
```

Note that `UiClickEvent` is **NOT** emitted automatically; you need to add a component to emit this event when `Pointer<Down>` is triggered, if you decide to make your own components.
```rust
// If we click on this node, it will emmit UiClick event on itself
UiClickEmitter::SELF,

// If we click on this node, it will emmit UiClick event from specified entity
UiClickEmitter::new(entity),
```

This component is really useful when creating complex components. You want `UiClickEvent` to be emmited from the top entity in a component, so users can listen to them. This component allows you to do exactly that.


Another components that you might find useful are:
```rust
// If it detects UiClick event for this entity it will despawn the specified entity, great for despawning routes
OnUiClickDespawn::new(entity),

// If it detects UiClick event for this entity it will run the closure, great for spawning routes
OnUiClickCommands::new(|commands| { commands.spawn(MyRoute); })
```