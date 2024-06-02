# Installation

Installing `Bevy_Lunex` is straightforward, just like any other Rust crate. Ensure that the version is `0.1.0-alpha` and not `0.0.X`.

Add the following to your `Cargo.toml`:

```toml
[dependencies]
bevy_lunex = { version = "0.1.0-alpha" }
```

Alternatively, you can use the latest version from the Git repository:

```toml
[dependencies]
bevy_lunex = { git = "https://github.com/bytestring-net/bevy_lunex" }
```

### Bevy

To avoid conflicts with `bevy_ui`, you can disable the default features and enable them manually. This prevents mixing different UI crates and reduces confusion. Refer to Bevy's [Cargo.toml](https://github.com/bevyengine/bevy/blob/main/Cargo.toml#L55) file for the complete list of features.

Add the following to your `Cargo.toml`:

```TOML
bevy = { version = "0.13.2", default_features = false, features = [
    # Core
    "bevy_core_pipeline",
    "multi-threaded",
    "bevy_winit",
    "bevy_audio",
    "bevy_sprite",
    "bevy_text",

    # Core formats
    "vorbis",
    "png",
] }
```

### Interactivity

To react to mouse events, enable the `picking` feature, which adds a custom `bevy_mod_picking` backend.

```TOML
bevy_lunex = { ..., features = ["picking"] }
```

Then, add `bevy_mod_picking` to your dependencies:

```TOML
bevy_mod_picking = { version = "0.18.2", default_features = false, features = [
    "debug",
    "selection",
    "highlight",
] }
```

Additionally, it's recommended (but not required) to include `bevy_eventlistener`:

```TOML
bevy_eventlistener = { version = "0.7.0" }
```