![image](https://github.com/bytestring-net/bevy_lunex/blob/main/promo/bevy_lunex.png?raw=true)

<div align="center">
  <a href="https://crates.io/crates/bevy_lunex"><img src="https://img.shields.io/crates/v/bevy_lunex?label=version&color=d69039"></a>
  <a href="https://crates.io/crates/bevy"><img src="https://img.shields.io/badge/v0.13.2-white.svg?label=bevy&color=bb86a5"></a>
  <a href="./LICENSE-MIT"><img src="https://img.shields.io/badge/License-Apache/MIT-white.svg?label=license&color=9fcec4"></a>
  <a href="https://deps.rs/crate/bevy_lunex"><img src="https://img.shields.io/badge/check-white.svg?label=deps&color=a0f6b9"></a>
  <a href="https://docs.rs/bevy_lunex"><img src="https://img.shields.io/docsrs/bevy_lunex/latest?color=8df7cb"></a>
</div>

#

Blazingly fast ***path based*** retained ***layout engine*** for Bevy entities, built around vanilla **Bevy ECS**. It gives you the ability to make ***your own custom UI*** using regular ECS like every other part of your app.

* **Resizable:** Lunex is designed to support ALL window sizes out of the box without deforming. The built in layout types react nicely and intuitively to aspect ratio changes.

* **Retained mode:** Unlike immediate mode GUI systems, Bevy_Lunex is a retained layout engine. This means the layout is calculated and stored, reducing the need for constant recalculations and offering potential performance benefits, especially for static or infrequently updated UIs.

* **ECS friendly:** Since it's built with ECS, you can extend or customize the behavior of your UI by simply adding or modifying components. The interactivity is done by regular systems and events.

* **2D + 3D UI:** One of the features of Bevy_Lunex is its support for both 2D and 3D UI elements, leveraging Bevy's `Transform` component. This opens up a wide range of possibilities for developers looking to integrate UI elements seamlessly into both flat and spatial environments. Diegetic UI is no problem.

* **Cursor styling:** Lunex offers a cursor API so you can style your cursor however you want! For interactions, we intagrate with [bevy_mod_picking](https://github.com/aevyrie/bevy_mod_picking), which is getting upstreamed into Bevy. Lunex also provides custom picking backend, you just need add `"picking"` feature.

##

![image](https://github.com/bytestring-net/bevy_lunex/blob/main/promo/image.png?raw=true)

*^ A recreation of ***Cyberpunk*** UI in ***Bevy***. [(Source code here)](https://github.com/IDEDARY/Bevypunk).*

## Documentation

For detailed read refer to [Bevy Lunex book](https://bytestring-net.github.io/bevy_lunex/).

For real-life example check out [Bevypunk source code](https://github.com/IDEDARY/Bevypunk).

## Versions

|  Bevy  |    Bevy Lunex   |
|--------|-----------------|
| 0.13.2 | 0.1.0 - latest  |
| 0.12.1 | 0.0.10 - 0.0.11 |
| 0.12.0 | 0.0.7 - 0.0.9   |
| 0.11.2 | 0.0.1 - 0.0.6   |

> ***Any version below 0.0.X is EXPERIMENTAL and is not intended for practical use***

## Contributing

Any contribution submitted by you will be dual licensed as mentioned below, without any additional terms or conditions. If you have the need to discuss this, please contact me.

## Licensing

Released under both [APACHE](./LICENSE-APACHE) and [MIT](./LICENSE-MIT) licenses. Pick one that suits you the most!
