# State Machine

Every Ui-Node has an internal state machine, represented by the `UiState` component.
States are not booleans — each state has a **weight** between `0.0` and `1.0`, which is smoothly
animated over time. The weights are then used to blend together the layouts and colors you
defined for each state, which makes transitions look fluid without any extra effort.

The state with the weight is called the *active state* and all others are *inactive*.

## Built-in states

- `UiBase` - The default state every node starts with (always weight `1.0` when no other state is active).
- `UiHover` - Enabled while the pointer is over the node, smoothly interpolates using configurable speeds.

> [!NOTE]
> There are additional states in the works (`UiSelected`, `UiClicked`, `UiIntro`, `UiOutro`), but
> they are **work in progress** and not yet wired up.

## Enabling a state

A state first needs to be **enabled** for the entity by adding its component. The most common one
is hover, which is driven by picking observers:

- `UiHover` - The state component with transition settings.
- `forward_speed` - How fast the weight goes towards `1.0` when enabled.
- `backward_speed` - How fast the weight falls back towards `0.0` when disabled.
- `instant` - Skip the animation entirely.

### Example

```rust, noplayground
ui.spawn((
    // Like this you can enable a state
    UiHover::new().forward_speed(20.0).backward_speed(4.0),
    // You can define layouts per state
    UiLayout::new(vec![
        (UiBase::id(), UiLayout::window().full()),
        (UiHover::id(), UiLayout::window().x(Rl(10.0)).full())
    ]),
    // You can define colors per state
    UiColor::new(vec![
        (UiBase::id(), Color::srgba(1.0, 0.0, 0.0, 0.8)),
        (UiHover::id(), Color::srgba(1.0, 1.0, 0.0, 1.0))
    ]),
    // ... Sprite, Text, etc.

// Add observers that enable/disable the hover state component
)).observe(hover_set::<PointerOver, true>)
  .observe(hover_set::<PointerOut, false>);
```

The `hover_set` utility is a ready-made observer that toggles the hover state on the entity it is
attached to. The generic `true`/`false` constant decides whether the state should be enabled or
disabled, so you pair it with `PointerOver` and `PointerOut` respectively.

> [!TIP]
> Hover events are automatically **duplicated to all children** of the observed entity, so hovering
> a parent node enables hover on the whole subtree. Attach the observers to the outermost node.

## How the blending works

For each frame, Lunex computes the rectangle of every state's layout and then normalizes the weights.
If, for example, hover is at `0.5`, the resulting node position and size are exactly halfway between
the `UiBase` layout and the `UiHover` layout. The same applies to colors, which are blended in HSLA
space (the hue is interpolated along the shortest arc).

If no state is active at all, the `UiBase` layout and color are used as fallback.

## Custom states

You can define your own states by implementing `UiStateTrait` for a component:

- `value()` - Returns the current weight, expected to be within `0.0 - 1.0`. Any smoothing
  should happen inside this function.

```rust, noplayground
#[derive(Component)]
struct UiWiggle {
    value: f32,
}

impl UiStateTrait for UiWiggle {
    fn value(&self) -> f32 {
        self.value
    }
}
```

You then reference the state by its `id()` when defining layouts and colors, the same way as
built-in states. To pipe the value into the state machine, use the generic system
`system_state_pipe_into_manager::<UiWiggle>` — it reads the component, writes the value into
`UiState` and triggers `RecomputeUiLayout` for the affected nodes.

```rust, noplayground
app.add_systems(Update, system_state_pipe_into_manager::<UiWiggle>);
```

> [!WARNING]
> The weight of `UiBase` is automatically balanced to `1.0 - (sum of all other states)`, so you
> never define it manually. If your custom state is at `1.0`, the base layout is fully faded out.
