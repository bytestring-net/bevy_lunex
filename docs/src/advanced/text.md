# Text

To render text, you use `UiText2dBundle` together with `Window` layout.
All you have to do is specify the position and the anchor of the text node.

You can disregard any size parameters, as they get overwritten by text-size.

For text-size, the provided `font_size` parameter is used, but instead of pixels it becomes `Rh` unit.
Currently it is hardcoded, but in the future you will be able to specify which unit to use.

```rust
// Link this widget
UiLink::<MainButtonUi>::path("Text"),

// Here we can define where we want to position our text within the parent node,
// don't worry about size, that is picked up and overwritten automaticaly by Lunex to match text size.
UiLayout::window().pos(Rl((5., 50.))).anchor(Anchor::CenterLeft).pack(),

// Add text
UiText2dBundle {
    text: Text::from_section("Hello world!",
        TextStyle {
            font: assets.load("font.ttf"),
            font_size: 60.0, // Currently hardcoded as Relative height (Rh) - so 60% of the node height
            color: Color::RED,
        }),
    ..default()
},
```