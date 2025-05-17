# Text 2D

Text rendering is done by using the Bevy's built-in `Text2d` component in conjunction with the `Window` ui layout.

- `UiLayout` - Specifies position and anchor only, size is ignored.
- `UiTextSize` - Specifies the height of the text in proportion to parent node.
- `Text2d` - Everything else works the same as normal Bevy 2D text rendering.

### Example

```rust, noplayground
ui.spawn((
    // Position the text using the window layout's position and anchor
    UiLayout::window().pos((Rh(40.0), Rl(50.0))).anchor(Anchor::CenterLeft).pack(),
    // This controls the height of the text, so 60% of the parent's node height
    UiTextSize::from(Rh(60.0)),
    // You can attach text like this
    Text2d::new("Button"),
    // Font size now works as "text resolution"
    TextFont {
        font: asset_server.load("fonts/Rajdhani.ttf"),
        font_size: 64.0,
        ..Default::default()
    },
));
```

> [!WARNING]
> `Text2d` component can ONLY be rendered with `Camera2d`. 3D text is a separate matter.

### How does it work?

When you spawn a `Text2d`, Lunex will wait until Bevy computes the text bounds (glyph size, font size, etc.).
After Bevy is done with the text, Lunex will take these values and put them inside `UiLayout::boundary::size` property
scaled together with `UiTextSize`. After the Ui layout is computed for the given frame, it will scale the `Transform`
so that the text fits into the node bounds.