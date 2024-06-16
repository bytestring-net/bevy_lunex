# Abstraction

As your user interface grows, it can become unmanageable. To address this, we will abstract our UI into higher-level components. This approach helps maintain organization and scalability.

## Project Structure

I recommend setting up your project with a structure similar to the following:

```
src
 |-- components
 |    |-- mod.rs
 |    |-- custom_button.rs
 |-- routes
 |    |-- mod.rs
 |    |-- my_route.rs
 |-- main.rs
```

We will create `components` and `routes` folders to contain our higher-level components. Each abstraction will have its own `.rs` file.

## Setting Up Components

In `components/mod.rs`, we will aggregate all respective component plugins into a single plugin.


```rust
// components/mod.rs

pub mod custom_button;
pub use custom_button::*;


// #=== ROUTE PLUGIN ===#
use bevy::prelude::*;

pub struct ComponentPlugin;
impl Plugin for ComponentPlugin {
    fn build(&self, app: &mut App) {
        app
            // Add each component plugin
            .add_plugins(CustomButtonPlugin);
    }
}
```

Similarly, `routes/mod.rs` will aggregate all route plugins.

```rust
// routes/mod.rs

pub mod my_route;
pub use my_route::*;


// #=== ROUTE PLUGIN ===#
use bevy::prelude::*;

pub struct RoutePlugin;
impl Plugin for RoutePlugin {
    fn build(&self, app: &mut App) {
        app
            // Add each route plugin
            .add_plugins(MyRoutePlugin);
    }
}
```

Finally, ensure these plugins are added in your `main.rs`.

```rust
// main.rs

mod components;
mod routes;

pub use bevy::prelude::*;
pub use bevy_lunex::prelude::*;
pub use {components::*, routes::*};

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, UiPlugin))
        .add_plugins(ComponentPlugin)
        .add_plugins(RoutePlugin)
        .run();
}
```

With this project structure in place, you can now focus on creating reusable components, making your codebase more organized and maintainable.