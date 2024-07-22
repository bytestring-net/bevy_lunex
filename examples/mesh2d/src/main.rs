use bevy::prelude::*;
use bevy_lunex::prelude::*;


fn main() {
    App::new()
        .add_plugins((DefaultPlugins, UiPlugin))
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut cmd: Commands, mut materials: ResMut<Assets<ColorMaterial>>) {

    // Spawn camera
    cmd.spawn((
        MainUi,
        Camera2dBundle {
            transform: Transform::from_xyz(0.0, 0.0, 1000.0),
            camera: Camera::default(),
            ..default()
        }
    ));

    // Spawn UiTree
    cmd.spawn((
        UiTreeBundle::<MainUi> {
            tree: UiTree::new2d("MyUiSystem"),
            ..default()
        },
        MovableByCamera,
    )).with_children(|ui| {

        // Spawn boundary node
        ui.spawn((
            UiLink::<MainUi>::path("Root"),
            UiLayout::boundary().pos1(Ab(20.0)).pos2(Rl(100.0) - Ab(20.0)).pack::<Base>(),
        ));

        // Spawn a color filled node
        ui.spawn((
            UiLink::<MainUi>::path("Root/Rectangle"),
            UiLayout::solid().size((Ab(1920.0), Ab(1080.0))).pack::<Base>(),
            UiMaterial2dBundle {
                material: materials.add(Color::srgb(1.0, 0.5, 0.5)),
                ..default()
            }
        ));
    });
}