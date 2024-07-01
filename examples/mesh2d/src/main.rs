use bevy::{prelude::*, sprite::{MaterialMesh2dBundle, Mesh2dHandle}};
use bevy_lunex::prelude::*;


fn main() {
    App::new()
        .add_plugins((DefaultPlugins, UiPlugin))
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut cmd: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<ColorMaterial>>) {

    cmd.spawn((
        MainUi,
        Camera2dBundle {
            transform: Transform::from_xyz(0.0, 0.0, 1000.0),
            camera: Camera::default(),
            ..default()
        }
    ));

    cmd.spawn((
        UiTreeBundle::<MainUi> {
            tree: UiTree::new("MyUiSystem"),
            ..default()
        },
        MovableByCamera,
    )).with_children(|ui| {

        ui.spawn((
            UiLink::<MainUi>::path("Root"),
            UiLayout::boundary().pos1(Ab(20.0)).pos2(Rl(100.0) - Ab(20.0)).pack::<Base>(),
        ));

        ui.spawn((
            UiLink::<MainUi>::path("Root/Rectangle"),
            UiLayout::solid().size((Ab(1920.0), Ab(1080.0))).pack::<Base>(),
            Element,
            Dimension::default(),
            MaterialMesh2dBundle {
                mesh: Mesh2dHandle(meshes.add(Rectangle { half_size: Vec2::splat(50.0) })),
                material: materials.add(Color::srgb(1.0, 0.5, 0.5)),
                ..default()
            }
        ));

    });

}