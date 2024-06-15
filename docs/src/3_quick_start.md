# Quick start

## Explanation

Lunex works by first creating an entity that will contain the future UI. This entity is called `UiTree` and has a component with the same name. Afterwards, any entity that will be part of that UI needs to be spawned as it's child.

## Boilerplate

```rust
use bevy_lunex::prelude::*;
```

Then we need to add `UiPlugin` to our app.

```rust
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)

        // This plugin is required for Lunex to work
        .add_plugins(UiPlugin)

        .run();
}
```

Thats it for the boilerplate!

## Start

Because the library supports a lot of different use cases, we need to specify what dimensions will the UI be rendered with.

Right now we want to use the window size, so we will use the default marker component and add it to our camera.
This will make the camera pipe it's size into our future `UiTree` which also needs to have the same marker applied.

```rust
commands.spawn((

    // Add this marker component provided by Lunex.
    MainUi,

    // Our camera bundle with depth 1000.0 because UI starts at `0` and goes up with each layer.
    Camera2dBundle { transform: Transform::from_xyz(0.0, 0.0, 1000.0), ..default() }
));
```

Now we need create our `UiTree` entity. The core components are `UiTree` + `Dimension` + `Transform`. The `UiTreeBundle` already contains these components for our ease of use.

The newly introduced `Dimension` component is used as the source size for the UI system. We also need to add the `MovableByCamera` component so our entity will receive updates from camera. The last step is adding the default `MainUi` marker as a generic.

```rust
commands.spawn((

    // This makes the UI entity able to receive camera data
    MovableByCamera,

    // This is our UI system
    UiTreeBundle::<MainUi>::from(UiTree::new("MainUi")),

)).with_children(|ui| {
    // Here we will spawn our UI as children
});
```

Now, any entity with `UiLayout` + `UiLink` spawned as a child of the `UiTree` will be managed as a UI entity. If it has a `Transform` component, it will get aligned based on the `UiLayout` calculations taking place in the parent `UiTree`. If it has a `Dimension` component then its size will also get updated by the `UiTree` output. This allows you to create your own systems reacting to changes in `Dimension` and `Transform` components.

You can add a `UiImage2dBundle` to the entity to add images to your widgets. Or you can add another `UiTree` as a child, which will use the computed size output in `Dimension` component instead of a `Camera` piping the size to it.

The generic in `pack::<S>()` represents state. For now leave it at `Base`, but when you later want to add hover animation use `Hover` instead.

```rust
ui.spawn((

    // Link the entity
    UiLink::<MainUi>::path("Root"),

    // Specify UI layout
    UiLayout::window_full().pos(Ab(20.0)).size(Rl(100.0) - Ab(40.0)).pack::<Base>(),
));

ui.spawn((

    // Link the entity
    UiLink::<MainUi>::path("Root/Rectangle"),

    // Specify UI layout
    UiLayout::solid().size(Ab((1920.0, 1080.0))).pack::<Base>(),

    // Add image to the entity
    UiImage2dBundle::from(assets.load("background.png")),
));
```

`UiLink` is what is used to define the the custom hierarchy. It uses `/` as the separator. If any of the names don't internally exist inside the parent `UiTree`, it will create them.

As you can see in the terminal (If you have enabled `debug` feature or added a `UiDebugPlugin`), the structure looks like this:
```rust
> MyUiSystem == Window [pos: (x: 0, y: 0) size: (x: 100%, y: 100%)]
    |-> Root == Window [pos: (x: 20, y: 20) size: (x: -40 + 100%, y: -40 + 100%)]
    |    |-> Rectangle == Solid [size: (x: 1920, y: 1080) align_x: 0 align_y: 0]
```
