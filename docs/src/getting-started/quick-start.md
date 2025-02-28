# Quick start

Now that we have everything setup, let's create some quick UI.

First, spawn a `UiLayoutRoot`. This is where our UI will start. You can specify the size of the UI viewport with `Dimension` component, but for 2D we don't want that. Instead we add `UiFetchFromCamera::<N>` with `N` being the index of our camera's `UiSourceCamera::<N>` component.

This will ensure that the `Camera -> Dimension -> UiLayout` pipeline will always be up to date.

```rust, noplayground
// Create UI
commands.spawn((
    // Initialize the UI root for 2D
    UiLayoutRoot::new_2d(),

    // Make the UI synchronized with camera viewport size
    UiFetchFromCamera::<0>,

)).with_children(|ui| {

    // ... Here we will spawn our UI

});
```

And now inside the `with_children` closure we will spawn a red rectange node.
This rectangle will be position exactly in the middle of our screen and
with width `200px` and height `50px`.

```rust, noplayground
ui.spawn((
    // You can name the entity
    Name::new("My Rectangle"),

    // Specify the position and size of the button
    UiLayout::window()
        .anchor(Anchor::Center) // Put the origin at the center
        .pos(Rl((50.0, 50.0)))  // Set the position to 50%
        .size((200.0, 50.0))    // Set the size to [200.0, 50.0]
        .pack(),

    // Color the sprite with red color
    UiColor::from(Color::srgb(1.0, 0.0, 0.0)),

    // Attach sprite to the node
    Sprite::from_image(asset_server.load("images/button.png")),

    // When hovered, it will request the cursor icon to be changed
    OnHoverSetCursor::new(SystemCursorIcon::Pointer),

// Interactivity is done through observers, you can query anything here
)).observe(|_: Trigger<Pointer<Click>>, mut exit: EventWriter<AppExit>| {
    
    // Close the app on click
    exit.send(AppExit::Success);
});
```

And thats it! You can of course do much more with the crate.
Continue reading to learn on how to spawn text nodes, enable animations and much more!