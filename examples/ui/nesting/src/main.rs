use bevy::prelude::*;
use bevy_lunex::prelude::*;
use bevy_vector_shapes::prelude::*;
use theme::ThemePlugin;

use crate::theme::UColor;

mod theme;
mod parameter;
mod div;

mod div_exp;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(Shape2dPlugin::default())
        .add_plugins(LunexUiPlugin)

        .add_plugins(ThemePlugin)

        .add_systems(Startup, setup)

        .add_systems(Update, (
            vector_rectangle_update,
        ).after(element_update))

        .run()
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>, mut window: Query<(&mut Window, Entity)>) {

    /*let mut pro = div::Div {
        class: div::DivClass::Break,
        placement: div::Placement::Fixed,
        content_size: Vec2::ZERO,
        nested_div: vec![div::TestBox::new().tiny().medium().large().into()],
    };

    pro.compute_content();

    println!("Content size: {}", pro.content_size);*/


    println!("Button: {:?}", div::Button::from("large, number: 7"));
    println!("Button: {:?}", div::Button::new().large().number(7.0));





    let btn = theme::UButton::new().primary();
    //println!("{:?}", btn);
    commands.spawn(btn);


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
    println!("{}", ui_tree.list());

    let _window = window.get_single_mut().unwrap();
    commands.entity(_window.1).insert((ui_tree, Transform::default(), bevy_lunex::prelude::Rectangle::default()));
}



pub fn build_interface (commands: &mut Commands, asset_server: &Res<AssetServer>, ui_tree: &mut UiTree) -> Result<(), LunexError> {

    let mut temporary_tree = UiTree::new("tmp");
    let tmp = &mut temporary_tree;

    
    let workspace = Widget::create(tmp, "workspace", RelativeLayout::new())?;

    let window = Widget::create(tmp, workspace.end("window"), WindowLayout {
        relative: Vec2::new(0., 0.),
        width_relative: 100.,
        height_relative: 100.,
        ..default()
    })?;


    // Merge the temporary tree to main ui tree

    ui_tree.merge(temporary_tree)?;



    // Spawn the image entity

    '_Fills: {
        commands.spawn(ImageElementBundle::new(window.clone(), &ImageParams::topleft().with_depth(1.0), asset_server.load("image.png"), Vec2::new(510.0, 200.0)));
        commands.spawn((
            ElementBundle::new(window.clone(), Element::fullfill()),
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