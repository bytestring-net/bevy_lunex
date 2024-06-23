# Linking

Lunex is somewhat similar to `bevy_rapier`, as it involves synchronizing with a separate `World`. This world is none other than the `UiTree` component. All UI data from its children are aggregated and synchronized into this "god-struct" that handles the layout computation.

### Under the Hood

What is `UiTree` under the hood, then? It is a specially modified hashmap variant called `NodeTree`. Its inner structure is almost identical to how your operating system storage works. You can think of `NodeTree` as an empty storage drive. You can insert so-called `Nodes`, which would be directories/folders in this analogy. Into each "folder," you can insert only one "file" (which is UI layout data in this case) and an unlimited number of nested subnodes.

Let's analyze this:
```rust
> MyUiSystem == Window [pos: (x: 0, y: 0) size: (x: 100%, y: 100%)]
    |-> Root == Window [pos: (x: 0, y: 0) size: (x: 100%, y: 100%)]
    |    |-> Rectangle == Solid [size: (x: 1920, y: 1080) align_x: 0 align_y: 0]
```

These are actually NOT entities but the so-called "NodeTree file system" printed out. It is inspired by GNU/Linux's `tree` command, which does exactly the same thing.

So `"MyUiSystem"` is your UiTree name, and its root layout is by default set to `window_full()`. Then you can see that this `UiTree` has a subnode called `"Root"`, which is created like this:

```rust
ui.spawn((
    UiLink::<MainUi>::path("Root"),         // Here we define the name of the node
    UiLayout::window_full().pack::<Base>(), // This is where we define the layout
));
```

Once this entity is created and picked up by Lunex, it creates the specific `"directory"` in the parent `UiTree` and then sends the corresponding data like layout with it as well. Once all data are prepared, Lunex will compute the correct layouts and send back this information to these "linked entities".

### Nesting

At some point, you will want to create a UI node inside another one. HTML does this too, but it is derived from syntax, like this:
```html
<div>
    <div />     // This div is inside the other div
</div>
```

You want this because the inner divs will affect the size of the outer div. In Lunex, you can nest by specifying a path. The `/` separator is used for this. This will create a `"Rectangle"` node inside of `"Root"`, as shown in the top tree printout.

```rust
ui.spawn((
    UiLink::<MainUi>::path("Root/Rectangle"),  // Here we define the name of the node
    UiLayout::window_full().pack::<Base>(),
));
```

But writing each path manually is not optimal. What if you want to change the structure? Will you change the paths of all the nodes then? Of course not.

```rust
let root = UiLink::<MainUi>::path("Root");  // Here we create the node link and store it
ui.spawn((
    root.clone(),                           // Here we add the link
    UiLayout::window_full().pack::<Base>(), // This is where we define the layout
));

ui.spawn((
    root.add("Background"), // We use the existing "root" link to create a chained link (same as "Root/Background")
    UiLayout::window_full().pack::<Base>(), // This is where we define the layout
));
```

### Which hierarchy to use

You will always want to use the Lunex hierarchy for all entities that should fall in the same UI system. We use Bevy's built-in hierarchy only to abstract our UI away, so we don't need to think about it.