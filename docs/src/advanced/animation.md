# Animation

Currently, the library supports only a single type of animation from `Base` -> `Hover`. However, this will change in the future as the animation framework has already been laid out and just needs to be implemented for all states.

To add hover animation to a UI node, you can utilize the following component:
```rust
UiAnimator::<Hover>::new().forward_speed(5.0).backward_speed(1.0)
```

With this component in place, you can then specify different UI properties using state generics, such as:
```rust
// Set base color to red
UiColor::<Base>::new(Color::RED),

// Set hover color to yellow
UiColor::<Hover>::new(Color::YELLOW),
```

You can also tween between two different layout positions by defining the hover layout like this:
```rust
// Hover layout specification
UiLayout::window_full().x(Rl(10.0)).pack::<Hover>(),

// Required to tween between states
UiLayoutController::default(),
```

When you need to synchronize animations on different nodes, consider using the pipe component that sends data to a specified entity:
```rust
// Pipe hover data to the specified entities
UiAnimatorPipe::<Hover>::new(vec![text, image]),
```

To receive this animation, make sure the specified entities have animator set to receiver mode:
```rust
UiAnimator::<Hover>::new().receiver(true),
```