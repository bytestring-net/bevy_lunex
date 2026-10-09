# 2D Usage

2D UI is the most common setup — the UI lives in the regular 2D render world and scales with your camera.

## Camera setup

First you need a `Camera2d` marked as a UI source. The `UiSourceCamera::<N>` component pairs the
camera with Ui-Trees that fetch from the same index `N`:

```rust, noplayground
commands.spawn((
    // This camera will become the source for all UI paired to index 0.
    Camera2d, UiSourceCamera::<0>,

    // Ui nodes start at 0 and move + on the Z axis with each depth layer.
    // This will ensure you will see up to 1000 nested children.
    Transform::from_translation(Vec3::Z * 1000.0),
));
```

> [!IMPORTANT]
> The `Vec3::Z * 1000.0` offset is not random — Ui-Node depth stacking starts at `0.0` and grows
> with nesting. Moving the camera back ensures deeply nested nodes are not clipped behind it.

## Root setup

Then you spawn the UI root, synchronized with the camera's viewport size via `UiFetchFromCamera`:

```rust, noplayground
commands.spawn((
    // Initialize the UI root for 2D
    UiLayoutRoot::new_2d(),

    // Make the UI synchronized with camera viewport size
    UiFetchFromCamera::<0>,
)).with_children(|ui| {
    // ... Here we will spawn our UI
});
```

> [!TIP]
> If your camera moves, spawn the root as a **child of the camera** instead — the UI then follows
> the camera around like a HUD. Keep `UiFetchFromCamera` so the root stays sized to the viewport.

Every child of the root becomes a Ui-Node. You attach visuals to the nodes directly — pick
whatever fits your use case:

- `Sprite` - The simplest option for images.
- `UiMeshPlane2d` + `MeshMaterial2d<ColorMaterial>` - A quad mesh reconstructed from the node's
  `Dimension` on demand, ideal for colored panels.
- Custom mesh - Full freedom for arbitrary geometry (see [Meshes](../3d/meshes.md) for the pattern).
- `Text2d` - Text rendering, see [Text](text.md).

```rust, noplayground
ui.spawn((
    Name::new("My Sprite"),
    // Give it some solid aspect ratio
    UiLayout::solid().size((1920.0, 1080.0)).pack(),
    // Give it a texture
    Sprite::from_image(asset_server.load("background.png")),
    // On hover change the cursor to this
    OnHoverSetCursor::new(SystemCursorIcon::Pointer),
))
.observe(|_: On<PointerClick>| info!("Click!"));
```

> [!NOTE]
> Lunex UI is just regular Bevy ECS — nodes are entities with `Transform`, so they can be picked,
> observed and animated like anything else in your app.

## Where to go next

- [Layouts](layouts.md) - The three available layout types.
- [Text](text.md) - 2D text rendering.
- [Interactivity](../interactivity.md) - Observers and pointer events.
