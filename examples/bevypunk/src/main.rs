use bevy::prelude::*;
use bevy_lunex::prelude::*;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins.set (
            WindowPlugin {
                primary_window: Some(Window {
                    title: "Bevypunk".into(),
                    mode: bevy::window::WindowMode::BorderlessFullscreen,
                    ..default()
                }),
                ..default()
            }
        ), UiPlugin::<NoData, NoData, MyWidget>::new()))
        //.add_plugins(UiDebugPlugin::<NoData, NoData, MyWidget>::new())
        .add_systems(PreStartup, presetup)
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands, assets: Res<AssetCache>, mut _materials: ResMut<Assets<StandardMaterial>>) {

    commands.spawn((
        MyWidget,
        Camera2dBundle {
            transform: Transform::from_xyz(0.0, 0.0, 1000.0),
            ..default()
        }
    ));

    commands.spawn((
        UiTreeBundle::<NoData, NoData, MyWidget> {
            tree: UiTree::new("MyWidget"),
            dimension: Dimension::new((1000.0, 1000.0)),
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..default()
        },
        MovableByCamera,
    )).with_children(|parent| {

        let root = UiLink::path("Root");
        parent.spawn((
            MyWidget,
            root.clone(),
            UiLayout::Window::FULL.pack(),
            //UiStack::new().direction(FlexDirection::Vertical),
        ));


        /* parent.spawn((
            MyWidget,
            root.new(),
            UiLayout::Div::new().pad(Abs::MD).margin(Abs::SM).br().pack(),
            /*UiText2dBundle {
                text: Text::from_section("hello world!",
                    TextStyle {
                        font: assets.font.clone(),
                        font_size: 60.0,
                        color: Color::RED,
                    }),
                ..default()
            }*/
            //UiImage2dBundle::from(assets.main_background.clone())
        )); */






        parent.spawn((
            MyWidget,
            UiLink::path("Root/Square"),
            UiLayout::Solid::new().size(Abs((1920.0, 1080.0))).cover(Cover::Full).pack(),
            UiImage2dBundle::from(assets.main_background.clone())
        ));

        parent.spawn((
            MyWidget,
            UiLink::path("Root/Board"),
            UiLayout::Solid::new().size(Abs((807.0, 1432.0))).align_x(Align(-0.8)).pack(),
            UiImage2dBundle::from(assets.main_board.clone())
        ));

    });

}


#[derive(Component, Debug, Default, Clone, PartialEq)]
pub struct MyWidget;

#[derive(Resource)]
pub struct AssetCache {
    pub font: Handle<Font>,
    pub font_bold: Handle<Font>,
    pub button: Handle<Image>,

    pub switch_base: Handle<Image>,
    pub switch_head: Handle<Image>,

    pub main_background: Handle<Image>,
    pub main_board: Handle<Image>,
    pub main_logo: Handle<Image>,
    pub settings_background: Handle<Image>,
}
fn presetup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(AssetCache {
        font: asset_server.load("fonts/rajdhani/Rajdhani-Medium.ttf"),
        font_bold: asset_server.load("fonts/rajdhani/Rajdhani-Bold.ttf"),
        button: asset_server.load("images/main_menu/button.png"),

        switch_base: asset_server.load("images/settings/switch_base.png"),
        switch_head: asset_server.load("images/settings/switch_head.png"),

        main_background: asset_server.load("images/main_menu/background.png"),
        main_board: asset_server.load("images/main_menu/board.png"),
        main_logo: asset_server.load("images/main_menu/bevypunk.png"),
        settings_background: asset_server.load("images/settings/background.png"),
    });
}
