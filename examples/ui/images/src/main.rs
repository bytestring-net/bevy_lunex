use bevy::prelude::*;
use bevy_lunex::prelude::*;
use bevy_vector_shapes::prelude::*;


fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(Shape2dPlugin::default())
        .add_plugins(LunexUiPlugin)

        .add_plugins(LunexUiDebugPlugin2D)

        .add_systems(Startup, setup)

        .add_systems(Update, (
            vector_rectangle_update,
        ).after(element_update))

        .run()
}
fn setup(mut commands: Commands, asset_server: Res<AssetServer>, mut window: Query<(&mut Window, Entity)>) {
    commands.spawn(
        Camera2dBundle {
            transform: Transform {
                translation: Vec3 { x: 0., y: 0., z: 1000. },
                ..default()
            },
            ..default()
        }
    );
    let mut ui_tree = UiTree::new("interface");
    build_interface(&mut commands, &asset_server, &mut ui_tree).unwrap();
    println!("{}", ui_tree.tree());

    let _window = window.get_single_mut().unwrap();
    commands.entity(_window.1).insert((ui_tree, Transform::default(), bevy_lunex::prelude::Size::default()));
}


pub fn build_interface (commands: &mut Commands, asset_server: &Res<AssetServer>, ui_tree: &mut UiTree) -> Result<(), LunexError> {

    let mut temporary_tree = UiTree::new("tmp");
    let tmp = &mut temporary_tree;

    
    let workspace = Widget::create(tmp, "workspace", RelativeLayout::new())?;

    
    let window1 = Widget::create(tmp, workspace.end("window1"), WindowLayout {
        relative: Vec2::new(5., 5.),
        width_relative: 30.,
        height_relative: 20.,
        ..default()
    })?;

    let window2 = Widget::create(tmp, workspace.end("window2"), WindowLayout {
        relative: Vec2::new(50., 5.),
        width_relative: 30.,
        height_relative: 20.,
        ..default()
    })?;

    let window3 = Widget::create(tmp, workspace.end("window3"), WindowLayout {
        relative: Vec2::new(5., 30.),
        width_relative: 30.,
        height_relative: 20.,
        ..default()
    })?;

    let window4 = Widget::create(tmp, workspace.end("window4"), WindowLayout {
        relative: Vec2::new(50., 30.),
        width_relative: 30.,
        height_relative: 20.,
        ..default()
    })?;

    let window5 = Widget::create(tmp, workspace.end("window5"), WindowLayout {
        relative: Vec2::new(5., 60.),
        width_relative: 30.,
        height_relative: 20.,
        ..default()
    })?;

    let window6 = Widget::create(tmp, workspace.end("window6"), WindowLayout {
        relative: Vec2::new(50., 60.),
        width_relative: 30.,
        height_relative: 20.,
        ..default()
    })?;


    // Merge the temporary tree to main ui tree
    ui_tree.merge(temporary_tree)?;

    // Spawn the image entity
    '_Fills: {
        commands.spawn(ImageElementBundle::new(window1.clone(), &ImageParams::topleft().with_depth(1.0), asset_server.load("image.png"), Vec2::new(510.0, 200.0)));
        commands.spawn((
            ElementBundle::new(window1.clone(), Element::fullfill()),
            VectorElementRectangle {
                color: Color::rgb_u8(40, 40, 40),
                corner_radii: Vec4::splat(10.0)
            },
        ));

        commands.spawn(ImageElementBundle::new(window2.clone(), &ImageParams::center().with_depth(1.0), asset_server.load("image.png"), Vec2::new(510.0, 200.0)));
        commands.spawn((
            ElementBundle::new(window2.clone(), Element::fullfill()),
            VectorElementRectangle {
                color: Color::rgb_u8(40, 40, 40),
                corner_radii: Vec4::splat(10.0)
            },
        ));

        commands.spawn(ImageElementBundle::new(window3.clone(), &ImageParams::topleft().with_depth(1.0).with_width(Some(100.0)), asset_server.load("image.png"), Vec2::new(510.0, 200.0)));
        commands.spawn((
            ElementBundle::new(window3.clone(), Element::fullfill()),
            VectorElementRectangle {
                color: Color::rgb_u8(40, 40, 40),
                corner_radii: Vec4::splat(10.0)
            },
        ));

        commands.spawn(ImageElementBundle::new(window4.clone(), &ImageParams::topleft().with_depth(1.0).with_width(Some(100.0)).with_height(Some(100.0)), asset_server.load("image.png"), Vec2::new(510.0, 200.0)));
        commands.spawn((
            ElementBundle::new(window4.clone(), Element::fullfill()),
            VectorElementRectangle {
                color: Color::rgb_u8(40, 40, 40),
                corner_radii: Vec4::splat(10.0)
            },
        ));

        commands.spawn(ImageElementBundle::new(window5.clone(), &ImageParams::center().at(100.0, 50.0).with_depth(1.0).with_scale(50.0), asset_server.load("image.png"), Vec2::new(510.0, 200.0)));
        commands.spawn((
            ElementBundle::new(window5.clone(), Element::fullfill()),
            VectorElementRectangle {
                color: Color::rgb_u8(40, 40, 40),
                corner_radii: Vec4::splat(10.0)
            },
        ));

        commands.spawn(ImageElementBundle::new(window6.clone(), &ImageParams::bottomcenter().with_depth(1.0), asset_server.load("image.png"), Vec2::new(510.0, 200.0)));
        commands.spawn((
            ElementBundle::new(window6.clone(), Element::fullfill()),
            VectorElementRectangle {
                color: Color::rgb_u8(40, 40, 40),
                corner_radii: Vec4::splat(10.0)
            },
        ));
    }

    Ok(())
}


/// Renders the widget
#[derive(Component)]
pub struct VectorElementRectangle {
    color: Color,
    corner_radii: Vec4
}
pub fn vector_rectangle_update (mut painter: ShapePainter, query: Query<(&Transform, &VectorElementRectangle)>) {
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