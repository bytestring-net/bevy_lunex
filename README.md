![image](https://github.com/bytestring-net/bevy_lunex/blob/main/promo/bevy_lunex.png?raw=true)

<div align="center">
  <a href="https://crates.io/crates/bevy_lunex"><img src="https://img.shields.io/crates/v/bevy_lunex?label=version&color=d69039"></a>
  <a href="https://crates.io/crates/bevy"><img src="https://img.shields.io/badge/v0.16-white.svg?label=bevy&color=bb86a5"></a>
  <a href="./LICENSE-MIT"><img src="https://img.shields.io/badge/License-Apache/MIT-white.svg?label=license&color=9fcec4"></a>
  <a href="https://deps.rs/crate/bevy_lunex"><img src="https://img.shields.io/badge/check-white.svg?label=deps&color=a0f6b9"></a>
  <a href="https://docs.rs/bevy_lunex"><img src="https://img.shields.io/docsrs/bevy_lunex/latest?color=8df7cb"></a>
</div>

#

Blazingly fast retained ***layout engine*** for Bevy entities, built around vanilla **Bevy ECS**. It gives you the ability to make ***your own custom UI*** using regular ECS like every other part of your app.

* **Any aspect ratio:** Lunex is designed to support ALL window sizes out of the box without deforming. The built in layout types react nicely and intuitively to aspect ratio changes.

* **Optimized:** Unlike immediate mode GUI systems, Bevy_Lunex is a retained layout engine. This means the layout is calculated and stored, reducing the need for constant recalculations and offering potential performance benefits, especially for static or infrequently updated UIs.

* **ECS focused:** Since it's built with ECS, you can extend or customize the behavior of your UI by simply adding or modifying components. The interactivity is done by regular systems and events.

* **Worldspace UI:** One of the features of Bevy_Lunex is its support for both 2D and 3D UI elements, leveraging Bevy's `Transform` component. This opens up a wide range of possibilities for developers looking to integrate UI elements seamlessly into both flat and spatial environments. Diegetic UI is no problem.

##

![image](https://github.com/bytestring-net/bevy_lunex/blob/main/promo/bevypunk_1.png?raw=true)

> *Try out the live WASM demo on [`Itch.io`](https://idedary.itch.io/bevypunk) (Limited performance & stutter due to running on a single thread). For best experience compile the project natively.*

## Syntax Example

This is an example of a clickable Button created from scratch using provided components.
Thanks to ECS, the syntax is highly modular with strong emphasis on components-per-functionality.
As you can see, it is no different from vanilla Bevy ECS.

```rust
// Create UI
commands.spawn((
    // Initialize the UI root for 2D
    UiLayoutRoot::new_2d(),
    // Make the UI synchronized with camera viewport size
    UiFetchFromCamera::<0>,
)).with_children(|ui| {

    // Spawn a button in the middle of the screen
    ui.spawn((
        Name::new("My Button"),
        // Specify the position and size of the button
        UiLayout::window().pos(Rl((50.0, 50.0))).size((200.0, 50.0)).pack(),
        // When hovered, it will request the cursor icon to be changed
        OnHoverSetCursor::new(SystemCursorIcon::Pointer),
    )).with_children(|ui| {
        
        // Spawn a child node but with a background
        ui.spawn((
            // You can define layouts for multiple states
            UiLayout::new(vec![
                // The default state, just fill the parent
                (UiBase::id(), UiLayout::window().full()),
                // The hover state, grow to 105% of the parent from center
                (UiHover::id(), UiLayout::window().anchor(Anchor::Center).size(Rl(105.0)))
            ]),
            // Enable the hover state and give it some properties
            UiHover::new().forward_speed(20.0).backward_speed(4.0),
            // You can specify colors for multiple states
            UiColor::new(vec![
                (UiBase::id(), Color::BEVYPUNK_RED.with_alpha(0.15)),
                (UiHover::id(), Color::BEVYPUNK_YELLOW.with_alpha(1.2))
            ]),
            // You can attach any form of rendering to the node, be it sprite, mesh or something custom
            Sprite {
                image: asset_server.load("images/button.png"),
                // Here we enable sprite slicing
                image_mode: SpriteImageMode::Sliced(TextureSlicer { border: BorderRect::all(32.0), ..default() }),
                ..default()
            },
            // Make sure it does not cover the bounding zone of parent
            Pickable::IGNORE,
        )).with_children(|ui| {

            // Spawn a text child node
            ui.spawn((
                // For text we always use window layout to position it. The size is computed at runtime from text bounds
                UiLayout::window().pos((Rh(40.0), Rl(50.0))).anchor(Anchor::CenterLeft).pack(),
                UiColor::new(vec![
                    (UiBase::id(), Color::BEVYPUNK_RED),
                    (UiHover::id(), Color::BEVYPUNK_YELLOW.with_alpha(1.2))
                ]),
                UiHover::new().forward_speed(20.0).backward_speed(4.0),
                // Here we specify the text height proportional to the parent node
                UiTextSize::from(Rh(60.0)),
                // You can attach text like this
                Text2d::new("Click me!"),
                TextFont {
                    font: asset_server.load("fonts/semibold.ttf"),
                    font_size: 64.0,
                    ..default()
                },
                // Make sure it does not cover the bounding zone of parent
                Pickable::IGNORE,
            ));
        });
    })
    // Utility observers that enable the hover state on trigger
    .observe(hover_set::<Pointer<Over>, true>)
    .observe(hover_set::<Pointer<Out>, false>)
    // Interactivity is done through observers, you can query anything here
    .observe(|_: Trigger<Pointer<Click>>| {
        println!("I was clicked!");
    });
});
```

## Documentation

- The Lunex Book: [`Bevy Lunex book`](https://bytestring-net.github.io/bevy_lunex/).

- Highly documented source code on Docs.rs: [`Docs.rs`](https://docs.rs/bevy_lunex/latest/bevy_lunex/).

- Highly documented production-ready example: [`Bevypunk example`](https://github.com/IDEDARY/Bevypunk).

## Contributing

Any contribution submitted by you will be dual licensed as mentioned below, without any additional terms or conditions. If you have the need to discuss this, please contact me.

## Licensing

Released under both [APACHE](./LICENSE-APACHE) and [MIT](./LICENSE-MIT) licenses. Pick one that suits you the most!
