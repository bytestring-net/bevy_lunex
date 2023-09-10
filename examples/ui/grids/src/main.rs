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
    let mut tmp = &mut temporary_tree;

    let workspace = Widget::create(&mut tmp, "workspace", RelativeLayout::new())?;

    let window = Widget::create(&mut tmp, &workspace.end("window"), WindowLayout {
        relative: Vec2::new(10., 10.),
        width_relative: 80.,
        height_relative: 80.,
        ..default()
    })?;



    let segment1 = GridSegment::splat_cells(GridCell::sized(Vec2::new(5.0, 5.0)), 2).add_gaps(5.0);
    let segment2 = GridSegment::splat_cells(GridCell::sized(Vec2::new(5.0, 5.0)), 5).add_gaps(5.0);
    let segment3 = GridSegment::splat_cells(GridCell::sized(Vec2::new(5.0, 5.0)), 3).add_gaps(5.0).with_scale(Some(100.0));
    let segment4 = GridSegment::splat_cells(GridCell::sized(Vec2::new(5.0, 5.0)), 7).add_gaps(5.0);

    let grid = Grid::new().with_segments(vec![segment1,segment2,segment3,segment4]).add_gaps(5.0).with_orientation(GridOrientation::Vertical);


    //let iter = grid.build_in(tmp, &window)?;
    let (_, iter) = grid.build_in_solid(tmp, &window.end("Grid"), SolidLayout::new())?;

    // Assign entities to grid cells
    for x in 0..iter.len() {
        for y in 0..iter[x].len() {
            commands.spawn((
                ElementBundle::new(&iter[x][y], Element::fullfill()),
                VectorElementRectangle {
                    color: Color::rgb_u8((x * 255/iter.len()) as u8, (x * 255/iter.len()) as u8, (y * 255/iter[x].len()) as u8),
                    corner_radii: Vec4::splat(0.0)
                },
            ));
        }
    }


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
                color: Color::rgb_u8(100, 100, 100),
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
