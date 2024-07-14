![image](https://github.com/bytestring-net/bevy_lunex/blob/main/promo/bevy_lunex.png?raw=true)

<div align="center">
  <a href="https://crates.io/crates/bevy_lunex"><img src="https://img.shields.io/crates/v/bevy_lunex?label=version&color=d69039"></a>
  <a href="https://crates.io/crates/bevy"><img src="https://img.shields.io/badge/v0.14.0-white.svg?label=bevy&color=bb86a5"></a>
  <a href="./LICENSE-MIT"><img src="https://img.shields.io/badge/License-Apache/MIT-white.svg?label=license&color=9fcec4"></a>
  <a href="https://deps.rs/crate/bevy_lunex"><img src="https://img.shields.io/badge/check-white.svg?label=deps&color=a0f6b9"></a>
  <a href="https://docs.rs/bevy_lunex"><img src="https://img.shields.io/docsrs/bevy_lunex/latest?color=8df7cb"></a>
</div>

#

Blazingly fast ***path based*** retained ***layout engine*** for Bevy entities, built around vanilla **Bevy ECS**. It gives you the ability to make ***your own custom UI*** using regular ECS like every other part of your app.

* **Any aspect ratio:** Lunex is designed to support ALL window sizes out of the box without deforming. The built in layout types react nicely and intuitively to aspect ratio changes.

* **Optimized:** Unlike immediate mode GUI systems, Bevy_Lunex is a retained layout engine. This means the layout is calculated and stored, reducing the need for constant recalculations and offering potential performance benefits, especially for static or infrequently updated UIs.

* **ECS focused:** Since it's built with ECS, you can extend or customize the behavior of your UI by simply adding or modifying components. The interactivity is done by regular systems and events.

* **Worldspace UI:** One of the features of Bevy_Lunex is its support for both 2D and 3D UI elements, leveraging Bevy's `Transform` component. This opens up a wide range of possibilities for developers looking to integrate UI elements seamlessly into both flat and spatial environments. Diegetic UI is no problem.

* **Custom cursor:** You can style your cursor with any image you want! Lunex also provides easy drop-in components for mouse interactivity.

##

![image](https://github.com/bytestring-net/bevy_lunex/blob/main/promo/bevypunk_1.png?raw=true)

![image](https://github.com/bytestring-net/bevy_lunex/blob/main/promo/bevypunk_3.jpeg?raw=true)

> *Try out the live WASM demo on [`Itch.io`](https://idedary.itch.io/bevypunk) or [`GitHub Pages`](https://idedary.github.io/Bevypunk/) (Limited performance & stutter due to running on a single thread). For best experience compile the project natively. You can find [source code here](https://github.com/IDEDARY/Bevypunk).*

## Syntax Example

This is an example of a clickable Button created from scratch using predefined components.
As you can see, ECS modularity is the focus here. The library will also greatly benefit from upcoming
BSN (Bevy Scene Notation) addition that Cart is working on.

```rust
commands.spawn((

	// #=== UI DEFINITION ===#

	// This specifies the name and hierarchy of the node
	UiLink::<MainUi>::path("Menu/Button"),

	// Here you can define the layout using the provided units (per state like Base, Hover, Selected, etc.)
	UiLayout::window().pos(Rl((50.0, 50.0))).size((Rh(45.0), Rl(60.0))).pack::<Base>(),


	// #=== CUSTOMIZATION ===#

	// Give it a background image
	UiImage2dBundle { texture: assets.load("images/button.png"), ..default() },

	// Make the background image resizable
	ImageScaleMode::Sliced(TextureSlicer { border: BorderRect::square(32.0), ..default() }),

	// This is required to control our hover animation
	UiAnimator::<Hover>::new().forward_speed(5.0).backward_speed(1.0),

	// This will set the base color to red
	UiColor<Base>::new(Color::RED),

	// This will set hover color to yellow
	UiColor<Hover>::new(Color::YELLOW),


	// #=== INTERACTIVITY ===#

	// This is required for hit detection (make it clickable)
	PickableBundle::default(),

	// This will change cursor icon on mouse hover
	OnHoverSetCursor::new(CursorIcon::Pointer),

	// If we click on this, it will emmit UiClick event we can listen to
	UiClickEmitter::SELF,
));
```

## Documentation

There is a Lunex book for detailed explanations about the concepts used in Lunex. You can find it here: [`Bevy Lunex book`](https://bytestring-net.github.io/bevy_lunex/).

For production ready example/template check out [`Bevypunk source code`](https://github.com/IDEDARY/Bevypunk).

## Versions

|  Bevy  |    Bevy Lunex   |
|--------|-----------------|
| 0.14.0 | 0.2.0 - 0.2.2   |
| 0.13.2 |      0.1.0      |
| 0.12.1 | 0.0.10 - 0.0.11 |
| 0.12.0 | 0.0.7 - 0.0.9   |
| 0.11.2 | 0.0.1 - 0.0.6   |

> ***Any version below 0.0.X is EXPERIMENTAL and is not intended for practical use***

## Contributing

Any contribution submitted by you will be dual licensed as mentioned below, without any additional terms or conditions. If you have the need to discuss this, please contact me.

## Licensing

Released under both [APACHE](./LICENSE-APACHE) and [MIT](./LICENSE-MIT) licenses. Pick one that suits you the most!
