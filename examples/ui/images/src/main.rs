use bevy::{prelude::*, window::PrimaryWindow};
use bevy_lunex::prelude::*;
use bevy_vector_shapes::prelude::*;

/// Empty struct in this example.
/// Normally used as storage for widget data.
#[derive(Component, Default)]
struct D;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(Shape2dPlugin::default())
        .add_plugins(LunexUiPlugin2D::<D>::new())
        //.add_plugins(LunexUiDebugPlugin2D::<D>::new())

        .add_systems(Startup, setup)

        .add_systems(Update, (
            vector_rectangle_update,
        ).after(LunexUiSystemSet2D))

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

    // Build the UI system
    build_interface(&mut commands, &asset_server, &mut tree).unwrap();
    println!("{}", tree.tree());
    
    // Append UI system to a window entity
    let window = window.single();
    commands.entity(window).insert(tree.bundle());
}
fn build_interface<T:Default>(commands: &mut Commands, asset_server: &Res<AssetServer>, ui_tree: &mut UiTree<T>) -> Result<(), LunexError> {

    let mut temporary_tree = UiTree::new("tmp");
    let tmp = &mut temporary_tree;
    
    let workspace = RelativeLayout::new().build_as(tmp, "workspace")?;

    let window1 = WindowLayout::empty()
        .rel(Vec2::new(5., 5.))
        .width_rel(40.0)
        .height_rel(25.0)
        .build_as(tmp, workspace.end("window1"))?;

    let window2 = WindowLayout::empty()
        .rel(Vec2::new(50., 5.))
        .width_rel(40.0)
        .height_rel(25.0)
        .build_as(tmp, workspace.end("window2"))?;

    let window3 = WindowLayout::empty()
        .rel(Vec2::new(5., 35.))
        .width_rel(40.0)
        .height_rel(25.0)
        .build_as(tmp, workspace.end("window3"))?;

    let window4 = WindowLayout::empty()
        .rel(Vec2::new(50., 35.))
        .width_rel(40.0)
        .height_rel(25.0)
        .build_as(tmp, workspace.end("window4"))?;

    let window5 = WindowLayout::empty()
        .rel(Vec2::new(5., 65.))
        .width_rel(40.0)
        .height_rel(25.0)
        .build_as(tmp, workspace.end("window5"))?;

    let window6 = WindowLayout::empty()
        .rel(Vec2::new(50., 65.))
        .width_rel(40.0)
        .height_rel(25.0)
        .build_as(tmp, workspace.end("window6"))?;

    // Merge the temporary tree to main ui tree
    ui_tree.merge(temporary_tree)?;

    // Spawn the image entity
    '_Fills: {
        commands.spawn(ImageElementBundle::new(&window1, ImageParams::topleft().with_depth(1.0), asset_server.load("image.png"), Vec2::new(510.0, 200.0)));
        commands.spawn((
            ElementBundle::new(window1, Element::fullfill()),
            VectorElementRectangle {
                color: Color::rgb_u8(30, 30, 30),
                corner_radii: Vec4::splat(10.0)
            },
        ));

        commands.spawn(ImageElementBundle::new(&window2, ImageParams::center().with_depth(1.0), asset_server.load("image.png"), Vec2::new(510.0, 200.0)));
        commands.spawn((
            ElementBundle::new(window2.clone(), Element::fullfill()),
            VectorElementRectangle {
                color: Color::rgb_u8(30, 30, 30),
                corner_radii: Vec4::splat(10.0)
            },
        ));

        commands.spawn(ImageElementBundle::new(&window3, ImageParams::topleft().with_depth(1.0).with_width(Some(100.0)), asset_server.load("image.png"), Vec2::new(510.0, 200.0)));
        commands.spawn((
            ElementBundle::new(window3, Element::fullfill()),
            VectorElementRectangle {
                color: Color::rgb_u8(30, 30, 30),
                corner_radii: Vec4::splat(10.0)
            },
        ));

        commands.spawn(ImageElementBundle::new(&window4, ImageParams::topleft().with_depth(1.0).with_width(Some(100.0)).with_height(Some(100.0)), asset_server.load("image.png"), Vec2::new(510.0, 200.0)));
        commands.spawn((
            ElementBundle::new(window4, Element::fullfill()),
            VectorElementRectangle {
                color: Color::rgb_u8(30, 30, 30),
                corner_radii: Vec4::splat(10.0)
            },
        ));

        commands.spawn(ImageElementBundle::new(&window5, ImageParams::center().at(100.0, 50.0).with_depth(1.0).with_scale(50.0), asset_server.load("image.png"), Vec2::new(510.0, 200.0)));
        commands.spawn((
            ElementBundle::new(window5, Element::fullfill()),
            VectorElementRectangle {
                color: Color::rgb_u8(30, 30, 30),
                corner_radii: Vec4::splat(10.0)
            },
        ));

        commands.spawn(ImageElementBundle::new(&window6, ImageParams::bottomcenter().with_depth(1.0), asset_server.load("image.png"), Vec2::new(510.0, 200.0)));
        commands.spawn((
            ElementBundle::new(window6, Element::fullfill()),
            VectorElementRectangle {
                color: Color::rgb_u8(30, 30, 30),
                corner_radii: Vec4::splat(10.0)
            },
        ));
    }

    Ok(())
}

/// Renders the widget
#[derive(Component)]
struct VectorElementRectangle {
    color: Color,
    corner_radii: Vec4
}
fn vector_rectangle_update (mut painter: ShapePainter, query: Query<(&Transform, &VectorElementRectangle)>) {
    for (transform, color) in &query {

        painter.set_translation(transform.translation);
        painter.set_scale(Vec3::splat(1.0));

        let ww = transform.scale.x;
        let hh = transform.scale.y;

        painter.color = color.color;
        painter.thickness = 1.0;
        painter.corner_radii = color.corner_radii;
        painter.rect(Vec2::new(ww, hh));
    }
}