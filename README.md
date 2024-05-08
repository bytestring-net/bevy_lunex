![image](https://github.com/bytestring-net/bevy-lunex/assets/49441831/41d0cf62-26fe-40d3-8ed6-23644108f28f)

<div align="center">
  <a href="https://crates.io/crates/bevy_lunex"><img src="https://img.shields.io/crates/v/bevy_lunex?label=version&color=d69039"></a>
  <a href="https://crates.io/crates/bevy"><img src="https://img.shields.io/badge/v0.13.2-white.svg?label=bevy&color=bb86a5"></a>
  <a href="./LICENSE-MIT"><img src="https://img.shields.io/badge/License-Apache/MIT-white.svg?label=license&color=9fcec4"></a>
  <a href="https://deps.rs/crate/bevy_lunex"><img src="https://img.shields.io/badge/check-white.svg?label=deps&color=a0f6b9"></a>
  <a href="https://docs.rs/bevy_lunex"><img src="https://img.shields.io/docsrs/bevy_lunex/latest?color=8df7cb"></a>
</div>

#

> [!CAUTION]
> This branch is not released yet and is still WIP.

Blazingly fast ***path based*** retained ***layout engine*** for Bevy entities, built around vanilla **Bevy ECS**. This library is intended to replace the existing `bevy_ui` crate, but nothing is stopping you from using them both at the same time.

It uses a combination of Bevy's built-in hierarchy and its own custom hierarchy to give you the freedom of control without the bloat or borrow checker limitations usually faced when creating UI.

It gives you the ability to make ***your own custom UI*** using regular ECS like every other part of your app.

***TLDR:*** It positions your entities as HTML objects for you, so you can slap custom rendering or images on them.

## Showcase

![image](https://github.com/bytestring-net/bevy-lunex/assets/49441831/c5b6ae89-aad0-4cc1-9fd1-299b6ab0a80a)

*^ A recreation of ***Cyberpunk*** UI in ***Bevy***. [(Source code here)](https://github.com/IDEDARY/Bevypunk).*

## Description

> [!NOTE]
> This library is EXPERIMENTAL. Judge yourself if it is a good fit for your project.

Bevy_Lunex is built on a simple concept: to use Bevy's ECS as the foundation for UI layout and interaction, allowing developers to manage UI elements as they would any other entities in their game or application as opposed to bevy_ui.

* **Path-Based Hierarchy:** Inspired by file system paths, this approach allows for intuitive structuring and nesting of UI elements. It's designed to make the relationship between components clear and manageable, using a syntax familiar to most developers, while also avoiding the safety restrictions Rust enforces (as they don't help but instead obstruct for UI).

* **Retained Layout Engine:** Unlike immediate mode GUI systems, Bevy_Lunex uses a retained layout engine. This means the layout is calculated and stored, reducing the need for constant recalculations and offering potential performance benefits, especially for static or infrequently updated UIs.

* **Built on top of ECS:** Since it's built with ECS, you can extend or customize the behavior of your UI by simply adding or modifying components. The scripting is also done by regular systems you are familiar with.

* **2D & 3D UI:** One of the features of Bevy_Lunex is its support for both 2D and 3D UI elements, leveraging Bevy's `Transform` component. This support opens up a wide range of possibilities for developers looking to integrate UI elements seamlessly into both flat and spatial environments.

## Workflow

First, we need to define a component, that we will use to mark all entities that will belong to our ui system.

```rust
#[derive(Component, Default)]
pub struct MyUiSystem;
```

Then we need to add `UiPlugin` with our marker component. The `NoData` generics are used if you need to store some data inside the nodes.

```rust
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(UiPlugin::<NoData, NoData, MyUiSystem>::new())
        .run();
}
```

By marking any camera with `MyUiSystem`, it will pipe its size into our future UI system entity.

```rust
commands.spawn((
    MyUiSystem,
    Camera2dBundle {
        transform: Transform::from_xyz(0.0, 0.0, 1000.0),
        ..default()
    }
));
```

Now we should create our entity with the UI system. The base componets are `UiTree` + `Dimension` + `Transform`. The `UiTreeBundle` already contains these components. The newly introduced `Dimension` component is used as the source size for the UI system. We also need to add the `MovableByCamera` component so our entity will receive updates from camera. The last step is adding our `MyUiSystem` type as a generic.

```rust
commands.spawn((
    UiTreeBundle::<NoData, NoData, MyUiSystem> {
        tree: UiTree::new("MyUiSystem"),
        ..default()
    },
    MovableByCamera,
)).with_children(|ui| {
    // Here we will spawn our UI in the next code block ...
});
```

Now, any entity with `MyUiSystem` + `UiLayout` + `UiLink` spawned as a child of the `UiTree` will be managed as a UI entity. If it has a `Transform` component, it will get aligned based on the `UiLayout` calculations taking place in the parent `UiTree`. If it has a `Dimension` component then its size will also get updated by the `UiTree` output. This allows you to create your own systems reacting to changes in `Dimension` and `Transform` components.

You can add a `UiImage2dBundle` to the entity to add images to your widgets. Or you can add another `UiTree` as a child, which will use the computed size output in `Dimension` component instead of a `Camera` piping the size to it.

```rust
ui.spawn((
    MyUiSystem,
    UiLink::path("Root"),
    UiLayout::Window::FULL.pos(Ab(20.0)).size(Rl(100.0) - Ab(40.0)).pack(),
));

ui.spawn((
    MyUiSystem,
    UiLink::path("Root/Rectangle"),
    UiLayout::Solid::new().size(Ab((1920.0, 1080.0))).pack(),
    UiImage2dBundle::from(assets.load("background.png")),
));
```

`UiLink` is what is used to define the the custom hierarchy. It uses `/` as the separator. If any of the names don't internally exist inside the parent `UiTree`, it will create them.

As you can see in the terminal (If you have added a `UiDebugPlugin`), the final structure looks like this:
```rust
> MyUiSystem == Window [pos: (x: 0, y: 0) size: (x: 100%, y: 100%)]
    |-> Root == Window [pos: (x: 20, y: 20) size: (x: -40 + 100%, y: -40 + 100%)]
    |    |-> Rectangle == Solid [size: (x: 1920, y: 1080) align_x: 0 align_y: 0]
```

Quite simple, isn't it? Best part is that by relying on components only, you are potentially able to hot-reload UI or even stream UI over the network. The downside is that by relying on strings to link entities, we are giving up some safety that Rust provides. But I am all for using the right tools for the right task. By putting away some safety, we can skip the bothersome bloat that would otherwise be required for such application.

### Nodes & Units

There are multiple nodes in `UiLayout`.
* `Boundary` - Defined by _point1_ and _point2_, it is not influenced by UI flow and is absolutely positioned.
* `Window` - Defined by _point_ and _size_, it is not influenced by UI flow and is absolutely positioned.
* `Solid` - Defined by _size_ only, it will scale to fit the parenting node. It is not influenced by UI flow.
* `Div` - Defined by _padding_ & _margin_. Dictates the UI flow. It uses styleform paradigm, very similar to HTML.

> [!WARNING]
> `Div` is not finished, it's WIP, please refrain from using it.

This library comes with several UI units. They are:

* `Ab` - Stands for absolute, usually `Ab(1)` = **1px**
* `Rl` - Stands for relative, it means `Rl(1.0)` == **1%**
* `Rw` - Stands for relative width, it means `Rw(1.0)` == **1%w**, but when used in *height* field, it will use *width* as source
* `Rh` - Stands for relative height, it means `Rh(1.0)` == **1%h**, but when used in *width* field, it will use *height* as source
* `Em` - Stands for size of symbol M, it means `Em(1.0)` == **1em**, so size **16px** if font size is **16px**
* `Sp` - Stands for remaining space, it's used as proportional ratio between margins, to replace alignment and justification. Only used by `Div`
* `Vp` - Stands for viewport, it means `Vp(1.0)` == **1v%** of the `UiTree` original size
* `Vw` - Stands for viewport width, it means `Vw(1.0)` == **1v%w** of the `UiTree` original size, but when used in *height* field, it will use *width* as source
* `Vh` - Stands for viewport height, it means `Vh(1.0)` == **1v%h** of the `UiTree` original size, but when used in *width* field, it will use *height* as source

> [!WARNING]
> `Sp` is not finished, it's WIP, please refrain from using it.

## Versions

|  Bevy  |    Bevy Lunex   |
|--------|-----------------|
| 0.13.2 | 0.1.0 - latest  |
| 0.12.1 | 0.0.10 - 0.0.11 |
| 0.12.0 | 0.0.7 - 0.0.9   |
| 0.11.2 | 0.0.1 - 0.0.6   |

> [!WARNING]
> Any version below 0.0.X is experimental and is not intended for practical use.

## Contributing

Any contribution submitted by you will be dual licensed as mentioned below, without any additional terms or conditions. If you have the need to discuss this, please contact me.

## Licensing

Released under both [APACHE](./LICENSE-APACHE) and [MIT](./LICENSE-MIT) licenses. Pick one that suits you the most!
