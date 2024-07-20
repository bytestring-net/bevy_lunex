# 2D & 3D

When creating a game with both 2D and 3D elements, you may want to combine these two worlds for better visual effects or more engaging gameplay.

This can be useful for example in first-person shooters that require heads-up displays (HUDs) to display information about the player.

To achieve this fusion of 2D and 3D, you'll need to follow these steps:

### 1. Set up a 2D camera
First, set up your project like any other 2D setup.

```rust
commands.spawn(
    MainUi,
    Camera2dBundle {
        transform: Transform::from_xyz(0.0, 0.0, 1000.0),
        ..default()
    }
);
```

### 2. Create a texture
Next, create a texture with the size of your viewport that will serve as the rendering target for your 3D camera.

```rust
// Create a texture resource that our 3D camera will render to
let size = Extent3d { width: 1920, height: 1080, ..default() };

// Create the texture
let mut image = Image {
    texture_descriptor: TextureDescriptor {
        label: None,
        size,
        dimension: TextureDimension::D2,
        format: TextureFormat::Bgra8UnormSrgb,
        mip_level_count: 1,
        sample_count: 1,
        usage: TextureUsages::TEXTURE_BINDING
            | TextureUsages::COPY_DST
            | TextureUsages::RENDER_ATTACHMENT,
        view_formats: &[]
    },
    ..default()
};

// Initiate the image
image.resize(size);

// Add our texture to asset server and get a handle
let render_image = asset_server.add(image);
```

### 3. Set up your 3D camera
Then, spawn your 3D camera anywhere, specifying the target and any additional settings you require.

```rust
// Spawn 3D camera
commands.spawn(
    Camera3dBundle {
        camera: Camera {
            // To make this camera run before 2D camera
            order: -1,

            // The render target handle
            target: render_image.clone().into(),

            // For transparency
            clear_color: ClearColorConfig::Custom(Color::rgba(0.0, 0.0, 0.0, 0.0)),
            ..default()
        },
        ..default()
    }
);
```

### 4. Spawn a UI node with ImageBundle
Finally, spawn a UI node with the new image texture using the `UiImage2dBundle` structure.

```rust
// Spawn 3D camera view
ui.spawn((
    root.add("Camera3d"),
    UiLayout::solid().size((1920.0, 1080.0)).scaling(Scaling::Fill).pack::<Base>(),
    UiImage2dBundle::from(render_image),
    PickingPortal, // You can add this component to send picking events through the viewport.
));
```

By following these steps, you can successfully merge the 2D and 3D worlds in your game.