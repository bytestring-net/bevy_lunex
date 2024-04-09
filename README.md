![image](https://github.com/bytestring-net/bevy-lunex/assets/49441831/41d0cf62-26fe-40d3-8ed6-23644108f28f)

<div align="center">
  <a href="https://crates.io/crates/bevy_lunex"><img src="https://img.shields.io/crates/v/bevy_lunex?label=version&color=d69039"></a>
  <a href="https://crates.io/crates/bevy"><img src="https://img.shields.io/badge/v0.13.2-white.svg?label=bevy&color=bb86a5"></a>
  <a href="./LICENSE-MIT"><img src="https://img.shields.io/badge/License-Apache/MIT-white.svg?label=license&color=9fcec4"></a>
  <a href="https://deps.rs/crate/bevy_lunex"><img src="https://img.shields.io/badge/check-white.svg?label=deps&color=a0f6b9"></a>
  <a href="https://docs.rs/bevy_lunex"><img src="https://img.shields.io/docsrs/bevy_lunex/latest?color=8df7cb"></a>
</div>

#

Blazingly fast ***path*** based retained ***layout engine*** for Bevy entities. It is built around vanilla **Bevy ECS**. This library is intended to replace the existing `bevy_ui` feature, but nothing is stopping you from using them both at the same time.

It uses combination of Bevy's built-in hierarchy and it's own custom hierarchy to give you the freedom of control without much bloat or extreme borrow checker limitations UIs usually have to face.

It gives you the ability to make ***your own custom UI*** using regular ECS like every other part of your app.

Features:
* Declarative and parametric positioning of widgets
* Set of basic units (Ab, Rl, Rw, Rh, Em, Sp, Vp, Vw, Vh)
* Both visual and console debug information
* 2D & 3D custom cursor

What it doesn't do:
* Adds visual styling to containers
* Introduces any rendering code

***TLDR:*** It positions your entities as HTML objects for you, so you can slap custom rendering or images on them.

## Showcase

![image](https://github.com/bytestring-net/bevy-lunex/assets/49441831/c5b6ae89-aad0-4cc1-9fd1-299b6ab0a80a)

<details><summary>Gif</summary>
  
<img src="promo/readme_cyberpunk.gif" alt="Cyberpunk gif"/>

</details>

*^ A recreation of ***Cyberpunk*** UI in ***Bevy***. [(Source code here)](https://github.com/IDEDARY/Bevypunk).*

## Description

*Note: This library is EXPERIMENTAL. I do not guarantee consistent updates. I'm developing it for my own personal use, so if I judge it has outlived it's use case, I will stop developing this project.*

## Workflow

First, we need to define a component, that we will use to mark all entities that will belong to our ui system.

```rust
#[derive(Component)]
pub struct MyUiSystem;
```

Then we need to add `UiPlugin` with our marker component. Generic at `NoData` is used if you need to store some data inside the nodes.

```rust
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(UiPlugin::<NoData, NoData, MyUiSystem>::new())
        .run();
}
```

By marking any camera with `MyUiSystem`, it will pipe it's size into our `UiTree` + `MyUiSystem` + `Dimension` + `MovableByCamera` entity.

```rust
commands.spawn((
    MyUiSystem,
    Camera2dBundle {
        transform: Transform::from_xyz(0.0, 0.0, 1000.0),
        ..default()
    }
));
```

`UiTreeBundle` contains `Dimension` component, that is used as the source size for the ui system. We also need to add `MovableByCamera` component and `MyUiSystem` as generic.

```rust
commands.spawn((
    UiTreeBundle::<NoData, NoData, MyUiSystem> { tree: UiTree::new("MyUiSystem"), ..default() },
    MovableByCamera,
)).with_children(|ui| {
    // Here we will spawn our UI in the next code block ...
});
```

Now, any entity with `MyUiSystem` + `UiLayout` + `UiLink` spawned as a child of the `UiTree` will be managed as a ui entity. If it has `Transform`, it will get aligned based on the `UiLayout` calculations taking place in parent `UiTree`. If it has `Dimension` component, it's size will also get updated by the `UiTree` output. This allows you to create your own systems reacting to changes in `Dimension` and `Transform`.

You can add `UiImage2dBundle` to the entity to apply image to your widgets. Or you can add another `UiTree` as a child, but instead of `Camera` piping the size to it, it will use the computed size output.

```rust
ui.spawn((
    MyUiSystem,
    UiLink::path("Root"),
    UiLayout::Window::FULL.pos(Abs(20.0)).size(Prc(100.0) - Abs(40.0)).pack(),
));

ui.spawn((
    MyUiSystem,
    UiLink::path("Root/Rectangle"),
    UiLayout::Solid::new().size(Abs((1920.0, 1080.0))).pack(),
    UiImage2dBundle::from(assets.load("background.png")),
));
```

`UiLink` is what is used to define the custom hierarchy. It uses `/` as the separator. If any of the names don't internally exist inside the parent `UiTree`, it will create them.

## Versions
|  Bevy  |    Bevy Lunex   |
|--------|-----------------|
| 0.13.2 | 0.1.0 - latest  |
| 0.12.1 | 0.0.10 - 0.0.11 |
| 0.12.0 | 0.0.7 - 0.0.9   |
| 0.11.2 | 0.0.1 - 0.0.6   |

## Contributing

Any contribution submitted by you will be dual licensed as mentioned below, without any additional terms or conditions.

## Licensing

Released under both [APACHE](./LICENSE-APACHE) and [MIT](./LICENSE-MIT) licenses, for the sake of compatibility with other projects. Pick one that suits you the most!
