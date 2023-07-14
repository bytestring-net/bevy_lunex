<h1 align="left">Bevy Lunex</h1>

[![License](https://img.shields.io/badge/License-MIT%20or%20Apache%202-blue.svg?label=license)](./LICENSE-MIT)

<p align="left">A novel path-based approach to UI built on top of existing Bevy features.</p>

![image](https://github.com/bytestring-net/bevy_lunex/assets/49441831/73d96dd1-d851-4a9f-9d58-11aba63e579d)
*A recreation of Cyberpunk 2077 menu in Bevy using Lunex*

## Table of Contents

- [Description](#description)
- [Usage](#usage)

## Description

Bevy_Lunex is an UI framework expanding on top of existing Bevy ECS features. It's purpose is to allow the developer to precisely define the behavior of widgets when window is being resized.
It's focus is on simlicity and the speed with how fast you can achieve AAA level UI. It achieves this by adding layout capabilites to vanilla entities so you can use them as UI elements.
It uses a path-based hierarchy to manage UI widgets. Pointers to these widgets are then passed to entities as components.
```
#ROOT
  |-> Main menu
  |    |-> Wallpaper
  |    |    |-> Background
  |    |-> Board widget
  |    |    |-> Logo
  |    |    |    |-> Logo Shadow
  |    |    |-> Button List
  |    |    |    |-> Continue
  |    |    |    |-> New Game
  |    |    |    |-> Load_Game
  |    |    |    |-> Settings
  |    |    |    |-> Credits
  |    |    |    |-> Additional Content
  |    |    |    |-> Quit Game
 ```

## Usage
First create a hierarchy struct that will hold all the recursive logic.
```rust
let mut system = Hierarchy::new();
```
To create a new widget in root directory you pass in the hierarchy, specify widget properties and the function returns a pointer. 
```rust
let widget_pointer = Widget::new(&mut system, "widget", Layout::Relative {
    relative_1: Vec2 { x: 0.0, y: 0.0 },
    relative_2: Vec2 { x: 100.0, y: 100.0 },
    ..Default::default()
}.wrap()).unwrap();
```
Once you have the pointer created you can pass the pointer to an entity as component to create interactive UI element. Here we add image to the widget.
```rust
commands.spawn ((
    widget_pointer,
    ImageInfo {
        width: 1920.0,
        height: 1080.0,
    },
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
To add logic to your UI elements, you use classic bevy system and query for your widgets. This function for example updates the position of an entity with image to match the calculated layout.
```rust
pub fn image_update(mut systems: Query<&mut Hierarchy>, mut query: Query<(&mut Widget, &ImageInfo, &mut Transform)>) {

    let mut system = systems.get_single_mut().unwrap();

    for (widget, imageinfo, mut transform) in &mut query {

        let dimensions = (system.width, system.height);
        let pos = widget.fetch_position(&mut system, "").unwrap(); //The widget will locate itself inside the hierarchy

        transform.translation.x = pos.point_1.x - dimensions.0/2.0;
        transform.translation.y = pos.point_2.y - dimensions.1/2.0;
        transform.scale.x = pos.width/imageinfo.width;
        transform.scale.y = pos.height/imageinfo.height;
    }
}
```

There are multiple layout options to choose from. With their combination and smart mixing user is able to define precisely how the layout should behave.

![image](https://github.com/bytestring-net/bevy_lunex/assets/49441831/180b773d-cbd3-4b3e-8d97-fbedde011e10)

