# Bevy Lunex

[![License](https://img.shields.io/badge/License-MIT%20or%20Apache%202-blue.svg?label=license)](./LICENSE-MIT)
[![crates.io](https://img.shields.io/crates/v/bevy_lunex.svg)](https://crates.io/crates/bevy_lunex)
[![Released API docs](https://docs.rs/bevy_lunex/badge.svg)](https://docs.rs/bevy_lunex)

A novel ***path*** based ***modular layout system*** built on top of **Bevy ECS**. It positions rectangles with user defined relations to achieve dynamic layout.
## === Showcase ===
![image](https://github.com/bytestring-net/bevy_lunex/assets/49441831/73d96dd1-d851-4a9f-9d58-11aba63e579d)

*^ A recreation of ***Cyberpunk 2077*** UI in ***Bevy***. It aligns SpriteBundles to values returned from Lunex achieving AAA level layout capabilites and modularity. [Source code here](https://github.com/IDEDARY/bevy-lunex-cyberpunk).*

## === Description ===

**Bevy_Lunex** is an layout framework with *endless amount of use cases*. The most prominet one is to use it as a *"building brick"* for **user interaface**.

However it can be used in **ANY** scenario where ***dynamic positioning is required***. For example:

* Already mentioned UIs, GUIs, HUDs, any on screen display.
* In-game UI, like floating labels next to a dropped item for example.
* Positioning and animating "elements", like sliding transitions.

In shortcut whenever you need to **position anything inside a rectagle** that is **NOT STATIC**, this library will be useful to you.

## === Workflow ===

Due to the nature of Rust, we had to come up with a **unique** way on how manage data. We decided to implement **hierarchy tree structure**, which is used in **UNIX file system**.

All data is stored in a master struct, called **Hierarchy**, which manages the inner tree. Custom pointers to these "**Branches**" are then passed to entities as **components**.

When needed, the pointers can **locate themselves** inside the tree and modify the data, thus **changing the behaviour** and result of the rectangle calculations taking place.
```
#ROOT
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

## === Usage ===
First create a hierarchy struct that will hold all the recursive data.
```rust
let mut system = Hierarchy::new();
```
### Creating widgets
To create a new widget in root directory you pass in the hierarchy, specify widget properties and the function returns a pointer. 
```rust
let widget_pointer = Widget::create(&mut system, "Widget", Box::Relative {
    relative_1: Vec2::new(0.0, 0.0),
    relative_2: Vec2::new(100.0, 100.0),
    ..Default::default()
}.pack()).unwrap();
```
### Spawning entities
Once you have the pointer created you can pass the pointer to an entity as component. Here we add image to the widget.
```rust
commands.spawn ((
    widget_pointer,
    SpriteBundle {
        texture: asset_server.load("image.png"),
        sprite: Sprite {
            anchor: Anchor::TopLeft,
            ..default()
        },
        ..default()
    }
));
```
### Querying for widgets
To add logic to your containers, you use bevy systems and query for your widgets. This function for example checks if cursor is inside a widget or not.
```rust
fn button_update(
    mut systems: Query<(&mut Hierarchy, &UIPlacement)>,
    cursors: Query<&Cursor>,
    mut widgets: Query<(&mut Widget)>,
    mouse_button_input: Res<Input<MouseButton>>
) {    
    //# Get Hierarchy and cursor
    let (mut system, placement) = systems.get_single_mut().unwrap();
    let cursor = cursors.get_single().unwrap();

    //# Loop through all widgets in the query
    for widget in &mut query {
        //# Check if the cursor is within the current widget boundaries
        if widget.is_within(&system, "", &vec_convert(cursor.position_world(), &placement.offset)).unwrap(){
            println!("Cursor is inside a widget!");            
        }
    }
}
```
### Layout defining

There are 3 options to pick from. With their combination you can define any layout. They are:
* **RELATIVE** || Defined from 2 points, as % of the parent container.
* **SOLID** || Defined as a ratio of width and height. Will scale to fit or fill parent.
* **WINDOW** || Defined as a point + width and height. Same as RELATIVE.

By combining containers of these 3 types, you can precisely define the position and dynamic bahvior.

## === Contributing ===

If you have an idea for improvement, start a discussion about it or create a pull request. If it is something aligned with the spirit of this repo I will try my best to merge it.

However, I do not want this repo to be an all-in-one solution for everything. It is middle-level framework supposed to be built upon.

## === Licensing ===

Released under both [APACHE](./LICENSE-APACHE) and [MIT](./LICENSE-MIT) licenses, for the sake of compatibility with other projects. Pick one that suits you the most!

