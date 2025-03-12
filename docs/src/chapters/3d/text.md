# Text 3D

Text rendering in 3D is done through `bevy_rich_text3d` crate which `Bevy_Lunex` re-exports. This is also gated
behind a `text3d` feature if you have disabled default features.

Similar to 2D text, we use `Text3d` component in conjunction with the `Window` ui layout.

- `UiLayout` - Specifies position and anchor only, size is ignored.
- `UiTextSize` - Specifies the height of the text in proportion to parent node.
- `Text3d` - Specifies the actual text.

> [!IMPORTANT]
> `Text3d` requires some necessary setup. You have to add these 2 components with some "default" values for it to work.
> - `MeshMaterial3d` - Required material to work. Recommended:
>     ```rust, noplayground
>     StandardMaterial {
>         base_color_texture: Some(TextAtlas::DEFAULT_IMAGE),
>         alpha_mode: AlphaMode::Blend,
>         unlit: true,
>         ..Default::default()
>     }
>     ```
> - `Mesh3d` - Required empty default `Mesh3d::default()` component to work.

### Example

```rust, noplayground
ui.spawn((
    Name::new("Panel"),
    // Set the layout of this mesh
    UiLayout::window().pos(Rl(50.0)).anchor(Anchor::Center).pack(),
    // This controls the height of the text, so 10% of the parent's node height
    UiTextSize::from(Rh(10.0)),
    // Set the text value
    Text3d::new("Hello 3D UI!"),
    // Style the 3D text
    Text3dStyling {
        size: 64.0,
        color: Srgba::new(1., 1., 1., 1.),
        align: TextAlign::Center,
        font: Arc::from("Rajdhani"),
        weight: Weight::BOLD,
        ..Default::default()
    },
    // Provide a material to this mesh
    MeshMaterial3d(materials.add(
        StandardMaterial {
            base_color_texture: Some(TextAtlas::DEFAULT_IMAGE),
            alpha_mode: AlphaMode::Blend,
            unlit: true,
            ..Default::default()
        }
    )),
    // Requires an empty mesh
    Mesh3d::default(),
));
```

> [!WARNING]
> `Text3d` component can ONLY be rendered with `Camera3d`. 3D text is a separate matter.

> [!NOTE]
> `bevy_rich_text3d` works a bit differently than `bevy_text`. When styling a text, you don't provide a font handle.
> Instead, the font must be loaded inside a `Text3dPlugin` when creating the plugin.
> 
> ```rust, noplayground
> UiLunexPlugins.set(Text3dPlugin {
>     // If we use custom fonts we need to load them here.
>     load_font_directories: vec!["assets/fonts".to_owned()],
>     // Load system fonts.
>     load_system_fonts: true,
>     ..default()
> })
> ```
