# Interactivity

Interactivity is done through observers. Let's recap on what observers are:

_Observers are a type of a one-shot system, that is run when specific event is triggered on specific entity._

We define these observers, which take `Trigger<E: Event>` that specify for which event it listens. Then we attach it
to a spawned entity (_local observer_).

We can listen to ANY event we want, even our own custom events. But in practise, the `Pointer<T>` events are the most common.
These events are related to `bevy_picking`, which are fired when for example a mouse cursor clicks when pointing at the entity.

- `Pointer<Click>`
- `Pointer<Over>`
- `Pointer<Out>`

These events also have metadata that you can access through the `Trigger` component, like for example which mouse button was pressed.

### Example

```rust, noplayground
ui.spawn((
    Name::new("Exit Button"),
    UiLayout::window()
        .anchor(Anchor::Center)
        .pos(Rl((50.0, 50.0)))
        .size((200.0, 50.0))
        .pack(),
    Sprite::from_image(asset_server.load("images/button.png")),

// Interactivity is done through observers, you can query anything here
)).observe(|_: Trigger<Pointer<Click>>, mut exit: EventWriter<AppExit>| {
    
    // Close the app on click
    exit.send(AppExit::Success);
});
```

