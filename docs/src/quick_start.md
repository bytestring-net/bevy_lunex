# Quick start

## Explanation

Lunex is first and foremost a worldspace layout engine, which means that **ALL** your UI entities exist in the same space as your game objects.

This for example means, that if you have a moving camera, you **HAVE TO** spawn your UI as children of the camera, otherwise your UI would stay where you spawned it. _(Most likely at [0,0,0])_

## Boilerplate

First import Lunex library

```rust
use bevy_lunex::prelude::*;
```

Then we add `UiPlugin` to our app.

```rust
fn main() {
    App::new()
        .add_plugins((DefaultPlugins, UiPlugin))
        .run();
}
```

Thats it for the boilerplate!

## Start

Lunex works by creating an entity that will contain all UI within it. This entity is called `UiTree` and has a component with the same name. Afterwards, any entity that will be part of that UI needs to be spawned as it's **direct** child.

Because the library supports a lot of different use cases, we **MUST** specify what source dimensions will the UI be be using. In most cases, we want it to take camera's viewport size.

To do that, you can tag your camera with the default `MainUi` marker provided by Lunex. If the library detects a camera with this marker, **ALL** `UiTree`s with the same tag will use this camera's size as size source.

```rust
commands.spawn((
    // Add this marker component provided by Lunex.
    MainUi,

    // Our camera bundle with depth 1000.0 because UI starts at `0` and goes up with each layer.
    Camera2dBundle { transform: Transform::from_xyz(0.0, 0.0, 1000.0), ..default() }
));
```

### UiTree

Now we need create our `UiTree` entity. Use the bundle below and attach `MovableByCamera` component so our `UiTree` will receive updates from our camera. The last step is adding the default `MainUi` marker as a generic.

```rust
commands.spawn((
    // This makes the UI entity able to receive camera data
    MovableByCamera,

    // This is our UI system
    UiTreeBundle::<MainUi>::from(UiTree::new2d("Hello UI!")),

)).with_children(|ui| {
    // Here we will spawn our UI as direct children
});
```

### UiNodes

Now, any entity with `UiLayout` + `UiLink` spawned as a child of the `UiTree` will be managed as a UI entity. If it has a `Transform` component, it will get aligned based on the `UiLayout` calculations taking place in the parent `UiTree`. If it has a `Dimension` component then its size will also get updated by the `UiTree` output.

This allows you to create your own systems reacting to changes in `Dimension` and `Transform` components.
You can very easily for example write a custom renderer system, that styles your nodes based on measurements in these components.

To quickly attach an image to our node, you can add a `UiImage2dBundle` to the entity to add images to your widgets.

The generic in `pack::<S>()` represents state. For now leave it at `Base`, but when you for example later want to add hover animation use `Hover` instead.

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

As you can see in the terminal (If you have enabled `debug` feature or added the `UiDebugPlugin`), the structure looks like this:
```rust
> MyUiSystem == Window [pos: (x: 0, y: 0) size: (x: 100%, y: 100%)]
    |-> Root == Window [pos: (x: 20, y: 20) size: (x: -40 + 100%, y: -40 + 100%)]
    |    |-> Rectangle == Solid [size: (x: 1920, y: 1080) align_x: 0 align_y: 0]
```

You can read more about this hierarchy in [**Linking**](advanced/linking.md)
