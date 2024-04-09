use bevy::prelude::*;
use bevy_lunex::prelude::*;


#[derive(Component)]
pub struct MyUiSystem;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(UiPlugin::<NoData, NoData, MyUiSystem>::new())
        .add_plugins(UiDebugPlugin::<NoData, NoData, MyUiSystem>::new())
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut cmd: Commands, mut _mat: ResMut<Assets<StandardMaterial>>, assets: Res<AssetServer>) {

    cmd.spawn((
        MyUiSystem,
        Camera2dBundle {
            transform: Transform::from_xyz(0.0, 0.0, 1000.0),
            camera: Camera::default(),
            ..default()
        }
    ));

    cmd.spawn((
        UiTreeBundle::<NoData, NoData, MyUiSystem> {
            tree: UiTree::new("MyUiSystem"),
            ..default()
        },
        MovableByCamera,
    )).with_children(|ui| {

        ui.spawn((
            MyUiSystem,
            UiLink::path("Root"),
            UiLayout::Window::FULL.pos(Abs(20.0)).size(Prc(100.0) - Abs(40.0)).pack(),
        ));

        ui.spawn((
            MyUiSystem,
            UiLink::path("Root/Rectangle"),
            UiLayout::Solid::new().size(Abs((1920.0, 1080.0))).pack(),
            UiImage2dBundle::from(assets.load("background.png")),
        ));

    });

}