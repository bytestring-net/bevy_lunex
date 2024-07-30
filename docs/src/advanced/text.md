# Text

To render text, you use `UiText2dBundle` together with `Window` layout.
All you have to do is specify the position and the anchor of the text node.

You can disregard any size parameters, as they get overwritten by text-size.

For text-size, the provided `font_size` parameter is used, but instead of pixels it becomes `Rh` unit. You can change this with `UiTextSize` component.

```rust
// Link this widget
UiLink::<MainButtonUi>::path("Text"),

// Here we can define where we want to position our text within the parent node,
// don't worry about size, that is picked up and overwritten automaticaly by Lunex to match text size.
UiLayout::window().pos(Rl((5., 50.))).anchor(Anchor::CenterLeft).pack::<Base>(),

// Add text
UiText2dBundle {
    text: Text::from_section("Hello world!",
        TextStyle {
            font: assets.load("font.ttf"),
            font_size: 60.0, // By default hardcoded as Relative height (Rh) - so 60% of the node height
            color: Color::RED,
        }),
    ..default()
},
```

You can also decouple the font size from the logical font size by adding this component. This new value will be used instead and native bevy font size will be used purely for rendering (font resolution).

```rust
UiTextSize::new().size(Rh(5.0)),
```