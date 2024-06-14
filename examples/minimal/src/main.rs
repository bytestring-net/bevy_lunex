use bevy::prelude::*;
use bevy_lunex::prelude::*;


#[derive(Component)]
pub struct MyUiSystem;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, UiGeneralPlugin))
        .add_plugins(UiPlugin::<MyUiSystem>::new())
        .add_plugins(UiDebugPlugin::<MyUiSystem>::new())
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut cmd: Commands, assets: Res<AssetServer>) {

    cmd.spawn((
        MyUiSystem,
        Camera2dBundle {
            transform: Transform::from_xyz(0.0, 0.0, 1000.0),
            camera: Camera::default(),
            ..default()
        }
    ));

    cmd.spawn((
        UiTreeBundle::<MyUiSystem> {
            tree: UiTree::new("MyUiSystem"),
            ..default()
        },
        MovableByCamera,
    )).with_children(|ui| {

        ui.spawn((
            UiLink::<MyUiSystem>::path("Root"),
            UiLayout::boundary().pos1(Ab(20.0)).pos2(Rl(100.0) - Ab(20.0)).pack::<Base>(),
        ));

        ui.spawn((
            UiLink::<MyUiSystem>::path("Root/Rectangle"),
            UiLayout::solid().size((Ab(1920.0), Ab(1080.0))).pack::<Base>(),
            UiImage2dBundle::from(assets.load("background.png")),
        ));

    });

}