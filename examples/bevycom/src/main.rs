mod boilerplate;
use boilerplate::*;
use bevy::prelude::*;
use bevy_lunex::prelude::*;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, UiPlugin::<NoData, NoData, MyWidget>::new()))
        .add_plugins(UiDebugPlugin::<NoData, NoData, MyWidget>::new())

        .add_systems(Startup, setup)

        .add_systems(Update, rotate_playercam)
        .add_systems(Update, zoom_playercam)
        .run();
}

fn setup(
    mut cmd: Commands,
    mut msh: ResMut<Assets<Mesh>>,
    mut mat: ResMut<Assets<StandardMaterial>>,
    assets: Res<AssetServer>,
) {
    cmd.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 1500000.0,
            range: 200000.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(0.0, 250.0, 250.0).with_rotation(Quat::from_euler(EulerRot::YXZ, 0.0, 30_f32.to_radians(), 0.0)),
        ..default()
    });
    let player = cmd.spawn(
        PbrBundle {
            //mesh: msh.add(Mesh::from(shape::Cube { size: 50.0 })),
            //material: mat.add(Color::rgb(0.8, 0.7, 0.6).into()),
            transform: Transform::from_xyz(0.0, 0.0, 0.0).with_scale(Vec3::new(1.0, 1.0, 1.0)),
            ..default()
        }
    ).id();
    let cam = cmd.spawn((
        Camera3dBundle::default(),
        PlayerCam {
            orbit: Vec3::new(0.0, 0.0, 0.0),
            distance: 800.0,
            sensitivity: Vec2::splat(0.1),
        }
    )).id();
    cmd.entity(player).push_children(&[cam]);

    for x in 0..1 {
        cmd.spawn((
            UiTreeBundle::<NoData, NoData, MyWidget> {
                transform: Transform::from_xyz(-400.0, 300.0, 0.0 + (200.0 * x as f32)),
                tree: UiTree::new("MyWidget"),
                ..default()
            },
            msh.add(Mesh::from(shape::Cube { size: 15.0 })),
            mat.add(Color::rgb(1.0, 0.0, 1.0)),
            Visibility::default(),
            ViewVisibility::default(),
    
        )).with_children(|parent| {
    
            let root = UiLink::path("Root");
            parent.spawn((
                MyWidget,
                root.clone(),
                UiLayout::Window::FULL.size(Abs((818.0, 965.0))).pack(),
                //UiMaterial3dBundle::from_image(&mut mat, assets.load("bevycom.png")),
            ));
    
            let head = root.add("Head");
            parent.spawn((
                MyWidget,
                head.clone(),
                UiLayout::Div::new().pad(Abs(20.0)).pack(),
                UiStack::new().direction(FlexDirection::Vertical),
                UiMaterial3dBundle::from_transparent_image(&mut mat, assets.load("bevycom_base_head.png")),
            ));
    
            parent.spawn((
                MyWidget,
                head.add("Icon"),
                UiLayout::Div::new().margin_r(Abs(20.0)).br().pack(),
                UiContent::new((115.0, 155.0)),
            ));
    
            parent.spawn((
                MyWidget,
                head.add("Rank"),
                UiLayout::Div::new().margin_b(Abs(10.0)).pack(),
                UiContent::new((100.0, 30.0)),
            ));
    
            parent.spawn((
                MyWidget,
                head.add("Name"),
                UiLayout::Div::new().margin_b(Abs(20.0)).pack(),
                UiContent::new((350.0, 45.0))
            ));
    
            let list = head.add("List");
            parent.spawn((
                MyWidget,
                list.clone(),
                UiLayout::Div::new().pad_y(Abs(10.0)).pack(),
                UiStack::new().gap_x(Abs(10.0))
            ));
    
            {
                parent.spawn((
                    MyWidget,
                    list.add("Missions"),
                    UiLayout::Div::new().pack(),
                    UiContent::new((200.0, 30.0))
                ));
    
                parent.spawn((
                    MyWidget,
                    list.add("Kills"),
                    UiLayout::Div::new().pack(),
                    UiContent::new((150.0, 30.0))
                ));
    
                parent.spawn((
                    MyWidget,
                    list.add("Status"),
                    UiLayout::Div::new().pack(),
                    UiContent::new((250.0, 30.0))
                ));
            }
    
        }); 
    }

}


#[derive(Component, Debug, Default, Clone, PartialEq)]
pub struct MyWidget;
