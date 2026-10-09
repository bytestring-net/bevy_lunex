# Flow Layout

The flow layout is a dynamic, flexbox-like layout model. Unlike the absolute layouts
(`Boundary`, `Window`, `Solid`), nodes with the `UiLayout::flow()` layout
**participate in the ui flow** - they interact with their siblings and can
react to their content.

```rust
use bevy_lunex::prelude::*;

commands.spawn((
    UiLayout::flow()
        .direction(UiFlowDirection::TopToBottom)
        .gap(Ab(10.0))
        .padding_all(Ab(20.0))
        .width(UiFlowSize::Grow)
        .height(Rl(50.0))
        .align(Align::CENTER)
        .justify(UiJustify::SpaceBetween)
        .pack(),
));
```

## Core concepts

A flow node is described by two things:

1. **How it takes up space in its parent's flow** - the `width`/`height` sizing and `margin`.
2. **How its children are arranged** - the `direction`, `gap`, `padding`, `align` and `justify`.

### Sizing

Each axis of a flow node is sized with one of the [`UiFlowSize`] variants:

| Size | Behavior |
|---|---|
| `UiFlowSize::Fit` | The node hugs its content (text, image or children). |
| `UiFlowSize::Grow` | The node claims one `Sp` share of the parent's leftover space, on top of its content. |
| `UiFlowSize::Fixed(...)` | The node is sized by an explicit [`UiValue`]. |

Sizing can be further constrained with `min_width`/`max_width`/`min_height`/`max_height` clamps.

```rust
UiLayout::flow()
    .width(UiFlowSize::Grow)          // claim a share of the available width
    .max_width(Ab(600.0))            // ...but never more than 600px
    .height(UiFlowSize::Fit)          // hug the content vertically
    .pack()
```

### Direction, gap, padding and margin

`direction` selects the layout direction of children: left-to-right,
right-to-left (inverted), top-to-bottom or bottom-to-top (inverted).
`gap` is spacing between children along that axis, `padding` is the spacing
between the node's bounding box and its children, and `margin` is the spacing
around the node itself within its parent's flow. All of them accept any
[`UiValue`], so they can be expressed in `Ab`, `Rl`, `Em`, `Vw`, `Sp`... units.

## The `Sp` unit (space)

All alignment and justification in flow layout is built on a single primitive:
the **`Sp` unit** - a proportional share of the *leftover space* (what remains
after all fixed sizes, gaps, paddings and fixed margins). `Sp` values are
resolved by the flow engine against the leftover space, shared proportionally
between all `Sp` claims of the children (margins *and* sizing):

```rust
// Two flexible children claiming 3:1 of the leftover width.
ui.spawn(UiLayout::flow().pack()).with_children(|ui| {
    ui.spawn(UiLayout::flow().width(Sp(3.0)).pack());
    ui.spawn(UiLayout::flow().width(Sp(1.0)).pack());
});

// A fixed base plus a flexible share: 50px + 1 share of the leftover.
ui.spawn(UiLayout::flow().width(Ab(50.0) + Sp(1.0)).pack());
```

- `Sp` in **sizing** acts as a flexible claim (like flex-grow): `Grow` is sugar
  for `Sp(1.0)` on top of the content size.
- `Sp` in **margins** acts as proportional spacing.
- With no leftover space (overflow), all `Sp` values resolve to `0`.
- Outside of flow layout, `Sp` evaluates to `0`.

## Alignment and justification are margins

The container's `align` and `justify` settings are not separate algorithms -
they **expand into default `Sp` margins inherited by the children**. A child
can override any side with its own `margin`; only undefined sides fall back to
the template.

**`align`** (cross axis, continuous `-1.0` to `1.0`) splits each child's
whitespace: `align: START` makes children inherit `margin_bottom: 1sp` (they
sit at the top), `CENTER` inherits `0.5sp` on both sides, `END` inherits
`margin_top: 1sp`.

**`justify`** (main axis) selects a margin template for the leftover space:

| Mode | Injected margins | Result |
|---|---|---|
| `Start` | none | children packed at the start, leftover after |
| `Center` | first `ml: 1sp`, last `mr: 1sp` | the block is centered |
| `End` | first `ml: 1sp` | the block is pinned to the end |
| `SpaceBetween` | all but first `ml: 1sp` | edges pinned, equal gaps between |
| `SpaceEvenly` | all `ml: 1sp`, last `mr: 1sp` | equal gaps everywhere (incl. edges) |
| `SpaceAround` | all `ml: 1sp` + `mr: 1sp` | half-size edges, full gaps between |

Children that claim leftover space through their *sizing* on an axis (`Grow`
or `Fixed` with `Sp`) do not inherit the template on that axis - they fill the
space instead, and only their own margins apply. This keeps the classic
"grow to fill" behavior working with any `align`/`justify` setting.

Because everything shares one pool, a child that defines its own `Sp` margins
joins the same distribution: with `justify: SpaceBetween`, a child with
`margin_left: Sp(2.0)` gets twice the gap of its siblings.

## Line wrapping

`.wrapping()` packs children onto multiple lines along the main axis, like
`flex-wrap: wrap` in CSS:

```rust
ui.spawn(UiLayout::flow().width(Ab(250.0)).wrapping().gap(Ab(10.0)).pack()).with_children(|ui| {
    for _ in 0..3 {
        ui.spawn(UiLayout::flow().width(Ab(100.0)).height(Ab(50.0)).pack());
    }
});
```

- Lines are packed greedily by the children's footprints (size plus fixed margins);
  an item wider than the whole container gets a line of its own.
- Each line resolves its **own `Sp` pool**: leftover space, justification margins
  and grow claims are computed per line, never across lines.
- A line's cross extent is its largest child footprint; `align` positions children
  within their line's extent.
- A `Fit` cross-sized wrapping container hugs the sum of its lines - the layout
  runs a bounded fixpoint pass so ancestors hug the wrapped size too.
- Wrapping requires the container's main-axis sizing to be resolvable top-down
  (not `Fit`): line packing needs to know the available extent.
- `.flipped()` stacks lines from the opposite edge - the first line sits at the
  cross-axis end, later lines wrap toward the start.

## Grid tracks

`.grid(...)` defines explicit tracks along the main axis, like CSS grid columns
(or rows, in vertical flows). Grid is a specialized wrapping mode: items are
placed sequentially into tracks, `n` per line, wrapping to the next line when
full:

```rust
ui.spawn(UiLayout::flow()
    .width(Ab(400.0)).gap(Ab(10.0))
    .grid([UiFlowSize::Grow, UiFlowSize::Grow])
    .pack())
.with_children(|ui| {
    for _ in 0..5 { ui.spawn(UiLayout::flow().height(Ab(50.0)).pack()); }
});
```

- `Fit` tracks hug their item's footprint (auto tracks), `Fixed` tracks are
  explicit lengths, and `Sp`/`Grow` tracks claim shares of the line's leftover
  space alongside the item's `Sp` margins (including the ones injected by
  `justify`).
- Items are stretched to fill their track (minus their fixed margins), floored
  at their minimum and capped at their maximum clamps.
- With `grid_wrap` enabled (the default) full lines wrap onto the next line;
  disabling it keeps every item on a single line, overflowing into implicit `Fit`
  tracks.
- Tracks run along the flow direction: use a vertical direction for row-based
  grids (the wrapped lines then stack along the cross axis).

## Units and the parent size

Flow parameters fully support the [`UiValue`] unit system, with one caveat:
relative units (`Rl`, `Rw`, `Rh`) in a node's sizing resolve against the
**parent's inner content box** (minus padding and gaps along the flow axis).
Because a `Fit` parent's size is derived from its children, relative-sized
children cannot contribute to it - they are excluded from the content-hugging
computation and resolve once the parent's size is known.

```rust
// Two children, each 50% of the parent's inner width - they exactly fill it.
ui.spawn(UiLayout::flow().gap(Ab(10.0)).pack()).with_children(|ui| {
    ui.spawn(UiLayout::flow().width(Rl(50.0)).pack());
    ui.spawn(UiLayout::flow().width(Rl(50.0)).pack());
});
```

`Sp` components of margins and sizing are likewise excluded from
content-hugging: a `Fit` parent has no leftover space, so `Sp` cannot
contribute to its hug.

## Coexistence with absolute layouts

Absolute layouts (`Window`, `Boundary`, `Solid`) inside a flow container keep
their absolute positioning - they are placed inside the flow container's
rectangle as usual and do not participate in the flow. A flow node whose
parent is *not* a flow container (for example a `Window` node) is sized inside
its parent according to its own `Fit`/`Grow`/`Fixed` sizing and placed through
the same margin templates (`justify` along, `align` across its direction).

The [`UiLayoutRoot`] itself acts as an implicit flow container (left to right,
no gap or padding) for its direct flow children.

## Text flow sizing

Attach the [`UiFlowText`] component to a flow text node to opt into flow-aware
text sizing. With `wrap` enabled, the width assigned by the flow engine is fed
back into the text's `TextBounds`, causing the text to re-wrap to its box. The
layout settles within one recompute cycle as the wrapped height feeds back in.

```rust
ui.spawn((
    UiLayout::flow().width(Rl(100.0)).pack(), // any width
    UiFlowText::wrapped(),
    Text2d::new("Some wrapping paragraph"),
));
```

## State machine integration

Flow parameters blend smoothly with the [state machine](state-machine.md).
A node's active flow configuration is the weighted blend of the flow layouts
of all its active states (gap, padding, margin, sizing values, `align`), so
states can animate layout parameters. Non-blendable fields (the `direction`,
the `justify` mode, or mismatching sizing kinds) snap to the highest-weight
state's value.

The node's *kind* (flow vs. absolute) is decided by its `UiBase` layout.

## How it works

1. **Margin injection** - each child's undefined margin sides receive the
   parent's `align`/`justify` default `Sp` templates.
2. **Bottom-up pass** - computes content-hugging sizes and minimum sizes from
   the leaves up, including the children's fixed margins.
3. **Top-down pass** - resolves relative sizing, then distributes space:
   on overflow, children shrink water-level (largest first, floored at their
   minimums); on leftover space, all `Sp` claims (margins and sizing) share
   it proportionally, re-normalized when maximum clamps bind.
4. **Position pass** - assigns each child's position from padding, resolved
   margins and child sizes. Inverted directions mirror the placement.

Any change that can affect the flow (layout edits, hierarchy changes, text
re-measurements, image loads) automatically triggers a recompute.
