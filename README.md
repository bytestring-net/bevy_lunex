![image](https://github.com/bytestring-net/bevy-lunex/assets/49441831/41d0cf62-26fe-40d3-8ed6-23644108f28f)

<div align="center">
  <a href="https://crates.io/crates/bevy_lunex"><img src="https://img.shields.io/crates/v/bevy_lunex?label=version&color=d69039"></a>
  <a href="https://crates.io/crates/bevy"><img src="https://img.shields.io/badge/v0.12.1-white.svg?label=bevy&color=bb86a5"></a>
  <a href="./LICENSE-MIT"><img src="https://img.shields.io/badge/License-Apache/MIT-white.svg?label=license&color=9fcec4"></a>
  <a href="https://deps.rs/crate/bevy_lunex"><img src="https://img.shields.io/badge/check-white.svg?label=deps&color=a0f6b9"></a>
  <a href="https://docs.rs/bevy_lunex"><img src="https://img.shields.io/docsrs/bevy_lunex/latest?color=8df7cb"></a>
</div>

# 

## NOTICE!
I'm currently working on 0.1.0 release which is a ground up rewrite of the engine.

Any version 0.0.X is considered experimental.

Note that I'm a student and this is not my primary project at the moment. Don't worry, development is ongoing, even if its slow.

#

Blazingly fast ***path*** based ***modular layout engine*** built on top of **Bevy ECS**. It calculates layout from user-defined ***rectangle positions*** and percentages. Works for ***all aspect ratios***. Uses retained style UI. (It may develop into full-fledged UI lib in the future)

*Note: I'm currently super busy with school, so the development during the school year is rather slow*

## === Showcase ===

![image](https://github.com/bytestring-net/bevy-lunex/assets/49441831/c5b6ae89-aad0-4cc1-9fd1-299b6ab0a80a)

<details><summary>Gif</summary>
  
<img src="promo/readme_cyberpunk.gif" alt="Cyberpunk gif"/>

</details>

*^ A recreation of ***Cyberpunk*** UI in ***Bevy***. [(Source code here)](https://github.com/IDEDARY/Bevypunk).*

## === Description ===

**Bevy-Lunex** is a layout engine based on defining and managing rectangles. It strives to be **clean**, **simple** and **intuitive** to the user. The most prominent use case is to use it as a *"building brick"* for your own **user interface**.

The core of this library is **pure math** layout engine, meaning **no styling** or **rendering** is currently included. That's where you come in. You can use the library to hook your own components and custom rendering to make it look exactly the way you want (May change in the future into full-fledged UI lib :D ).

By attaching entities to coordinates returned by **Bevy Lunex**, you can abstract complex positioning logic away from you. Take a look at these examples:
* **[`Bevypunk`](https://github.com/IDEDARY/Bevypunk)** - *Made by attaching images to Bevy Lunex rectangles and animating them*.
* **[`Stardawn`](https://github.com/IDEDARY/stardawn)** - *Used [bevy_vector_shapes](https://github.com/james-j-obrien/bevy_vector_shapes) to render resizable dynamic elements*.

Currently the most up-to-date practices and workflow can be found in the **[`Bevypunk`](https://github.com/IDEDARY/Bevypunk)** project.

## === Workflow ===
<details><summary>Expand</summary>

### --- Usage ---

Due to the nature of Rust, we had to come up with a **unique** way how to manage data. We decided to implement **hierarchy tree structure**, which is used in **UNIX file system**.

All data is stored in a master struct, called "**UiTree**", which manages all layout data. The **"UiTree"** is composed of "**UiBranches**", where each branch represents a rectangle and they can be nested inside each other. **"Widgets"** are custom smart pointers containing a *"path"* to the corresponding nested **"UiBranch"**. **"Widgets"** are **components** and are spawned as entity.

When needed, the **"Widget"** can *fetch* **"UiBranch"** inside the **"UiTree"** and return a mutable borrow. From the borrow you can modify the layout data, thus **changing the behaviour** and the result of the rectangle calculations taking place.
This is the way to get around the *Rust's borrow checker*.
```
> UI
  |-> Main_menu
  |    |-> Background
  |    |-> Board
  |    |    |-> Logo
  |    |    |-> Buttons
  |    |    |    |-> Continue
  |    |    |    |-> New_Game
  |    |    |    |-> Load_Game
  |    |    |    |-> Settings
  |    |    |    |-> Credits
  |    |    |    |-> Additional_Content
  |    |    |    |-> Quit_Game
 ```
^ This is a **"UiTree"** structure printed out in a terminal. Each item displayed here is **"UiBranch"**. Look for example at the *"Board"* branch, in which are nested *"Logo"* and *"Buttons"* branches.

### --- Tree creation ---

First, create a **"UiTree"** struct that will hold all the layout data managed recursively.
We also need to specify the generic, which is optional field each widget can have and store data in.
```rust
let mut tree: UiTree<MyData> = UiTree::new("UI");
```

### --- Layout definition ---
To create a new **"Widget"** in the root directory you pass in the **"UiTree"**, specify widget properties and the function returns the smart pointer. 
```rust
let widget: Widget = WindowLayout::empty()
    .rel(Vec2::splat(10.0))
    .size_rel((80.0, 80.0))
    .build_as(&mut tree, "widget")?;
```

### --- Logic binding ---
Once you have the **"Widget"** created, you can pass it to an entity as a component together with other components like **"Image"**. Here we use **"ImageElementBundle"**, which is the same as **"SpriteBundle"**, but has extra fields for **"Widget"** and **"Element"**. Element component is used when you need to attach a visual entity to a widget, like text or image.
```rust
commands.spawn((
    ImageElementBundle::new(
        widget,
        &ImageParams::default(),
        asset_server.load("button.png"),
        Vec2::new(1280.0, 250.0)),
    ButtonHighlightEffect::new(Color::GOLD),
));
```
In this example, we also passed another component called **"ButtonHighlightEffect"**, which we will define in the next section.

### --- Logic definition ---
To add logic to your **"Widgets"**, you use Bevy systems. In this example, we will create a system that will tint the sprite to a certain colour if a cursor hovers over the **"Widget"** First we define the component with color data. Then we define the system that will query **"UiTree"**, **"Cursor"** and our components. Add the logic and we are done.
```rust
#[derive(Component)]
pub struct ButtonHighlightEffect (pub Color);

fn button_highlight_effect_update<T:Component + Default>(
    trees: Query<&UiTree<T>>,
    cursors: Query<&Cursor>, 
    mut query: Query<(&Widget, &mut Sprite, &ButtonHighlightEffect)>
) {
    for tree in trees {
        for (widget, mut sprite, color) in &mut query {

            if !widget.fetch(&tree).unwrap().is_visible() {return;}

            let mut trigger = false;
            for cursor in &cursors {
                if widget.contains_position(&tree, &cursor.position_world().invert_y()).unwrap() {
                    trigger = true;
                    break;
                }
            }

            if trigger{
                sprite.color = color.0;
            } else {
                sprite.color = Color::WHITE;
            }
        }
    }
}
```
### --- Last ---
Don't forget to add the system to the app.
```rust
.add_systems(Update, button_highlight_effect_update::<T>)
```
You need to spawn the **"UiTree"** we created in the first step as an entity so we can query for it.
Look into examples how to inject the component into existing window to bind it to window size.
```rust
commands.spawn(tree);
```

### --- Layout options ---
There are 3 main layout options to pick from. With their combination, you can define any setup. They are:
* **RELATIVE** || Defined from 2 points, as % of the parent container.
* **SOLID** || Defined as a ratio of width and height. Will scale to fit or fill parent.
* **WINDOW** || Defined as a point + width and height. Same as RELATIVE.

By nesting branches of these 3 types, you can precisely define the position and layout behaviour.

</details>

## === Versions ===
|  Bevy  |   Bevy Lunex  |
|--------|---------------|
| 0.12.1 |     0.0.10    |
| 0.12.0 | 0.0.7 - 0.0.9 |
| 0.11.2 |    <= 0.0.6   |

## === Contributing ===

Any contribution submitted by you will be dual licensed as mentioned below, without any additional terms or conditions.

## === Licensing ===

Released under both [APACHE](./LICENSE-APACHE) and [MIT](./LICENSE-MIT) licenses, for the sake of compatibility with other projects. Pick one that suits you the most!
