# Components

Components (Not Bevy ECS componets, but UI components) bundle certain UI behavior under a single entity. Example can be **Button**, **Text input**, **Calendar**, **Minimap**, etc.

## Creating a Component

Begin by creating a new `.rs` file in the `components` folder. First, define a public component that will serve as the abstraction.

```rust
// components/custom_button.rs

/// When this component is added, a UI system is built
#[derive(Component)]
pub struct CustomButtom {
    // Any fields we want to interact with should be here.
    text: String,
}
```

Best practice is that all components should be sandboxed. For that reason we need to define a new marker component, that will be used ONLY for UI inside this button component (instead of `MainUi`).

```rust
// components/custom_button.rs

/// Marker struct for the sandboxed UI
#[derive(Component)]
struct CustomButtonUi;
```


Next, create a system that builds the component UI when the component is added. This system will insert the `UiTree` component into the same entity and spawn the UI elements as children.

```rust
// components/custom_button.rs

/// System that builds the route when the component is added
fn build_route(mut commands: Commands, assets: Res<AssetServer>, query: Query<Entity, Added<CustomButtom>>) {
    for entity in &query {
        commands.entity(entity).insert((
            // Insert this bundle into the entity that just got the CustomButtom component
            // Note that CustomButtonUi is used here instead of MainUi
            UiTreeBundle::<CustomButtonUi>::from(UiTree::new("CustomButton")),

        // Now spawn the UI as children
        )).with_children(|ui| {
            // Spawn some UI nodes
            ui.spawn((
                // Link this widget
                // Note that CustomButtonUi is used here instead of MainUi
                UiLink::<CustomButtonUi>::path("Image"),

                // Add layout
                UiLayout::window_full().pack::<Base>(),

                // Give it a background image
                UiImage2dBundle {
                    texture: assets.load("images/button.png"),
                    sprite: Sprite { color: Color::RED, ..default() },
                    ..default()
                },

                // Give the texture 9-slice tilling
                ImageScaleMode::Sliced(TextureSlicer { border: BorderRect::square(32.0), ..default() }),
            ))
        });
    }
}
```

Finally, add the system to a plugin.

```rust
// components/custom_button.rs

pub struct CustomButtonPlugin;
impl Plugin for CustomButtonPlugin {
    fn build(&self, app: &mut App) {
        app
            // Add Lunex plugins for our sandboxed UI
            .add_plugins(UiGenericPlugin::<CustomButtonUi>::new())

            // NOTE! Systems changing the UI need to run before UiSystems::Compute
            // or they will not get picked up by change detection.
            .add_systems(Update, build_route.before(UiSystems::Compute));
    }
}
```

Don't forget to add this plugin in `component/mod.rs`.

## Spawning a component

To spawn the component, you have to spawn it as UI node of another UI system, either a route or another component.

```rust
// Spawning the component
ui.spawn((
    UiLink::<MainUi>::path("Button"),
    UiLayout::window().size(Rl(50.0)).pack::<Base>(),

    CustomButton {
        text: "PRESS ME!".to_string(),
    },
));
```

To despawn the component, call `.despawn_recursive` on the spawned entity.

```rust
// Despawning the component
commands.entity(component_entity).despawn_recursive();
```