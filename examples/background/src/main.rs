use bevy::{prelude::*, window::PrimaryWindow};
use bevy_lunex::prelude::*;

/// Empty struct in this example.
/// Normally used as storage for widget data.
#[derive(Component, Default)]
struct D;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(LunexUiPlugin2D::<D>::new())
        .add_plugins(LunexUiDebugPlugin2D::<D>::new())

        .add_systems(Startup, setup)

        .run()
}
fn setup(mut commands: Commands, asset_server: Res<AssetServer>, window: Query<Entity, (With<Window>, With<PrimaryWindow>)>) {
    // Spawn camera
    commands.spawn(
        Camera2dBundle {
            transform: Transform {
                translation: Vec3::new(0.0, 0.0, 100.0),
                ..default()
            },
            ..default()
        }
    );

    // Create UI system
    let mut tree = UiTree::<D>::new("interface");

    // Build the UI widget system
    let widget = SolidLayout::new().with_size(510.0, 200.0).with_scaling(SolidScale::Fill).build(&mut tree).unwrap();
    
    // Spawn the Element widget with the image
    commands.spawn(ImageElementBundle::new(&widget, ImageParams::center().with_depth(1.0), asset_server.load("image.png"), Vec2::new(510.0, 200.0)));
    
    // Append UI system to a window entity
    let window = window.single();
    commands.entity(window).insert(tree.bundle());
}