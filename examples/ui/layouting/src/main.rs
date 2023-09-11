use bevy::prelude::*;
use bevy_lunex::prelude::*;
use bevy_vector_shapes::prelude::*;


fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(Shape2dPlugin::default())
        .add_plugins(LunexUiPlugin)

        .add_systems(Startup, setup)

        .add_systems(Update, (
            vector_rectangle_update,
        ).after(element_update))

        .run()
}

fn setup(mut commands: Commands) {
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
    build_interface(&mut commands, &mut ui_tree).unwrap();
    println!("{}", ui_tree.generate_map_debug());
    commands.spawn (ui_tree);
}



pub fn build_interface (commands: &mut Commands, ui_tree: &mut UiTree) -> Result<(), LunexError> {

    let mut temporary_tree = UiTree::new("tmp");
    let tmp = &mut temporary_tree;

    
    let workspace = Widget::create(tmp, "workspace", RelativeLayout::new())?;


    let window = Widget::create(tmp, workspace.end("window"), WindowLayout {
        relative: Vec2::new(10., 10.),
        width_relative: 50.,
        height_relative: 50.,
        ..default()
    })?;



    // Merge the temporary tree to main ui tree

    ui_tree.merge(temporary_tree)?;



    // Spawns the draw entities last

    '_Fills: {

        commands.spawn((
            ElementBundle::new(workspace.clone(), Element::fullfill()),
            VectorElementRectangle {
                color: Color::rgb_u8(48, 52, 70),
                corner_radii: Vec4::splat(0.0)
            },
        ));

        commands.spawn((
            ElementBundle::new(window.clone(), Element::fullfill()),
            VectorElementRectangle {
                color: Color::rgb_u8(200, 200, 200),
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
