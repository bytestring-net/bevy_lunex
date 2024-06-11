# Routes

In the context of a user interface, a "route" refers to a specific page of content, similar to HTML. Examples of routes might include the **Main Menu**, **Settings**, **Inventory**, etc.

## Creating a Route

Begin by creating a new `.rs` file in the `routes` folder. First, define a public component that will serve as the abstraction for the route.

```rust
// routes/my_route.rs

/// When this component is added, a UI system is built
#[derive(Component)]
pub struct MyRoute;
```

Next, create a system that builds the route when the component is added. This system will insert the `UiTree` component into the same entity and spawn the UI elements as children.

```rust
// routes/my_route.rs

/// System that builds the route when the component is added
fn build_route(mut commands: Commands, assets: Res<AssetServer>, query: Query<Entity, Added<MyRoute>>) {
    for entity in &query {
        commands.entity(entity).insert((
            // Insert this bundle into the entity that just got the MyRoute component
            UiTreeBundle::<MainUi>::from(UiTree::new("MyRoute")),
        // Now spawn the UI as children
        )).with_children(|ui| {
            // Spawn some UI nodes
            ui.spawn((
                UiLink::<MainUi>::path("Background"),
                UiLayout::solid().size((1920.0, 1080.0)).scaling(Scaling::Fill).pack::<Base>(),
                UiImage2dBundle::from(assets.load("images/background.png")),
            ));
        });
    }
}
```

Finally, add the system to a plugin.

```rust
// routes/my_route.rs

pub struct MyRoutePlugin;
impl Plugin for MyRoutePlugin {
    fn build(&self, app: &mut App) {
        app
            // NOTE! Systems changing the UI need to run before UiSystems::Compute
            // or they will not get picked up by change detection.
            .add_systems(Update, build_route.before(UiSystems::Compute));
    }
}
```

Don't forget to add this plugin in `routes/mod.rs`.

## Spawning a route

To spawn the route, simply call:

```rust
// Spawning the route
commands.spawn((
    MyRoute,
    MovableByCamera,    // Marks this ui to receive Transform & Dimension updates from camera size
));
```

To despawn the route, call `.despawn_recursive` on the spawned entity.

```rust
// Despawning the route
commands.entity(route_entity).despawn_recursive();
```

With this setup, you can effectively manage different UI routes within your application, keeping your codebase organized and maintainable.