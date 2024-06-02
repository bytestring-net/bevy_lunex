# Cursor

Lunex provides a custom API for styling a cursor within your Bevy application.

This feature works by spawning a cursor atlas image alongside a special `Cursor2d` component as a child of a 2D camera.

To use the custom cursor styling, we need to expand our `Camera2d` entity as follows:

```rust
# fn setup(mut commands: Commands, assets: Res<AssetServer>, mut atlas_layout: ResMut<Assets<TextureAtlasLayout>>){
commands.spawn((
    MainUi,
    Camera2dBundle { transform: Transform::from_xyz(0.0, 0.0, 1000.0), ..default() }
)).with_children(|camera| {

    // Spawn cursor entity
    camera.spawn ((

        // Here we can map different native cursor icons to texture atlas indexes and sprite offsets
        Cursor2d::new().native_cursor(false)
            .register_cursor(CursorIcon::Default, 0, (14.0, 14.0))
            .register_cursor(CursorIcon::Pointer, 1, (10.0, 12.0))
            .register_cursor(CursorIcon::Grab, 2, (40.0, 40.0)),

        // Add a SpriteSheetBundle to the cursor
        SpriteSheetBundle {
            texture: assets.load("cursor.png"),

            // Define the texture atlas layout
            atlas: TextureAtlas {
                layout: atlas_layout.add(TextureAtlasLayout::from_grid(Vec2::splat(80.0), 3, 1, None, None)),
                index: 0,
            },

            // Modify the scale of the cursor
            transform: Transform { scale: Vec3::new(0.45, 0.45, 1.0), ..default() },

            // Set the anchor to top-left
            sprite: Sprite {
                anchor: bevy::sprite::Anchor::TopLeft,
                ..default()
            },
            ..default()
        },

        // Make the raycaster ignore this entity to prevent the cursor from blocking clicks
        Pickable::IGNORE,
    ));
});
# }
```

When creating a `Cursor2d` component, you can use the `native_cursor()` method to specify whether the cursor should exist as an entity within the game world or be injected into the `Winit` crate as a custom cursor sprite. (Note: This feature is currently a work in progress, and enabling it only hides the sprite for now.)

![Cursor](../images/cursor.png)

By default, spawning the cursor entity will hide the native system cursor unless `native_cursor(true)` is set.

Additionally, you must register each cursor icon with its respective texture atlas indices and sprite offsets to define the appearance and positioning of different cursor states.

Finally, to prevent the cursor from interfering with clicking events, we add the `Pickable::IGNORE` component. This ensures that the cursor sprite does not block any button interactions or other clickable elements in the UI.
