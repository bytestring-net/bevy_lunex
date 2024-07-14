# Cursor

Lunex provides a custom abstraction for a cursor related features within your Bevy application.

This is achieved by moving all logic to a an entity that is spawned as a child of a `Camera2d`.

It is required that you spawn this entity, otherwise picking won't work.

### Required components

You need to spawn these components for 

```rust
// This is the main component
Cursor2d::new(),
// This is required so that the sprite doesn't block our picking raycaster
Pickable::IGNORE,
// This is required so that the sprite doesn't block our picking raycaster
PointerBundle::new(PointerId::Custom(pointer::Uuid::new_v4())),
```

### Styling components

If you want to attach custom image to your cursor, you have to attach texture atlas to the entity.
You will need to have all the icons in a image strip like this.

![Cursor](../images/cursor.png)


```rust
// Specify the texture atlas properties
TextureAtlas {
    // ...
},
// Specify the image to load 
SpriteBundle {
    // ...
},
```

### Gamepad support

To bind a cursor to a gamepad, you have to add this component:

```rust
GamepadCursor::new(0),
```

If you want the cursor to accept both Mouse and Gamepad inputs, you have to create an additional
system that listens to recent input events and based on them "removes" or "adds" this component.

Currently, there is only 1 mode supported and that is `Free` which means you just use your stick to move
the cursor around. There is no "jumping" yet.

However, it is planned to add `Snap` mode, which makes the cursor "jump" and snap to the next node in input direction.

## Example

Here's an example of how to set up a custom cursor with gamepad control:

```rust
# fn setup(mut commands: Commands, assets: Res<AssetServer>, mut atlas_layout: ResMut<Assets<TextureAtlasLayout>>){
commands.spawn((
    MainUi,
    Camera2dBundle { transform: Transform::from_xyz(0.0, 0.0, 1000.0), ..default() }
)).with_children(|camera| {

    // Spawn 2D camera
    commands.spawn(camera()).with_children(|camera| {

        // Spawn cursor
        camera.spawn ((

            // Here we can map different native cursor icons to texture atlas indexes and sprite offsets
            Cursor2d::new()
                .set_index(CursorIcon::Default, 0, (14.0, 14.0))
                .set_index(CursorIcon::Pointer, 1, (10.0, 12.0))
                .set_index(CursorIcon::Grab, 2, (40.0, 40.0)),

            // Here we specify that the cursor should be controlled by gamepad 0
            GamepadCursor::new(0),

            // This is required for picking to work
            PointerBundle::new(PointerId::Custom(pointer::Uuid::new_v4())),
            
            // Add texture atlas to the cursor
            TextureAtlas {                                           // Size 80x80, 3 columns, 1 row
                layout: atlas_layout.add(TextureAtlasLayout::from_grid(UVec2::splat(80), 3, 1, None, None)),
                index: 0,
            },
            SpriteBundle {
                texture: assets.load("cursor.png"),
                transform: Transform { scale: Vec3::new(0.45, 0.45, 1.0), ..default() },
                sprite: Sprite {
                    color: Color::YELLOW.with_alpha(2.0),
                    anchor: Anchor::TopLeft,
                    ..default()
                },
                ..default()
            },

            // Make the raycaster ignore this entity, we don't want our cursor to block clicking
            Pickable::IGNORE,
        ));
    });
});
# }
```
