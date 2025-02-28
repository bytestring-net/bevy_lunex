# Installation & Setup

Adding `Bevy_Lunex` to your project is straightforward, just like any other Rust crate.

Add the following to your `Cargo.toml`:

```toml
[dependencies]
  bevy_lunex = { version = "*" }
```

Alternatively, you can use the latest bleeding edge version from the Git repository:

```toml
[dependencies]
  bevy_lunex = { git = "https://github.com/bytestring-net/bevy_lunex" }
```

## Project Setup

You have to add the `UiLunexPlugin` to your application.

```rust, noplayground
fn main() -> AppExit {
    App::new()
        // Add necessary plugins
        .add_plugins((DefaultPlugins, UiLunexPlugin))
        .run()
}
```

Next you have to spawn your camera. Your main camera must have the `UiSourceCamera::<N>` component, with `N` being a constant from `0..3` range.

> [!NOTE]
> The purpose of this is that if you are creating a splitscreen game, you can have up to 4 cameras.
> This component tells the UI which camera's viewport size to use as the root node size.

> [!TIP]
> If you need more indexes, you can add `UiLunexIndexPlugin::<N>` for said index manually.

```rust, noplayground
fn spawn_camera(mut commands: Commands) {
    // Spawn the camera
    commands.spawn((

        // This camera will become the source for all UI paired to index 0.
        Camera2d, UiSourceCamera::<0>,
        
        // Ui nodes start at 0 and move + on the Z axis with each depth layer.
        // This will ensure you will see up to 1000 nested children.
        Transform::from_translation(Vec3::Z * 1000.0),
        
        // Explained in # Chapters/Debug-Tooling section of the book
        RenderLayers::from_layers(&[0, 1]),
    ));
}
```
