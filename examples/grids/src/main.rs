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
fn setup(mut commands: Commands, window: Query<Entity, (With<Window>, With<PrimaryWindow>)>) {
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
    build_interface(&mut commands, &mut tree).unwrap();
    println!("{}", tree.tree());
    
    // Append UI system to a window entity
    let window = window.single();
    commands.entity(window).insert(tree.bundle());
}
fn build_interface (commands: &mut Commands, ui_tree: &mut UiTree<D>) -> Result<(), LunexError> {

    let mut temporary_tree = UiTree::new("tmp");
    let tmp = &mut temporary_tree;

    let workspace = RelativeLayout::new().build_as(tmp, "workspace")?;

    let window = WindowLayout::empty()
        .rel(Vec2::splat(10.0))
        .size_rel((80.0, 80.0))
        .build_as(tmp, workspace.end("window"))?;

    let segment1 = GridSegment::splat_cells(GridCell::sized(Vec2::new(5.0, 5.0)), 11).add_gaps(2.0);
    let segment2 = GridSegment::splat_cells(GridCell::sized(Vec2::new(5.0, 5.0)), 5).add_gaps(5.0);
    let segment3 = GridSegment::splat_cells(GridCell::sized(Vec2::new(5.0, 5.0)), 3).add_gaps(7.0).with_scale(Some(100.0));
    let segment4 = GridSegment::splat_cells(GridCell::sized(Vec2::new(5.0, 5.0)), 7).add_gaps(5.0);

    let grid = Grid::new().with_segments(vec![segment1, segment2, segment3, segment4]).add_gaps(1.0).with_orientation(GridOrientation::Horizontal);

    let wgrid = if true {

        // This method builds the grid to fill 100% of the given window. Great for navbars and tabs.
        // Bad for invetories when you want the cells to have constant size regradless of cell count.
        grid.build_in(tmp, &window)?

    } else {

        // This method builds the grid inside a new "buffer" widget. The script will ensure that cells will have same size no
        // matter the number of cells generated. Great for slots and player inventories.
        let (_, wgrid) = grid.build_in_window(tmp, window.end("Grid"), WindowLayout::new())?;

        wgrid
    };

    // Assign entities to grid cells
    for x in 0..wgrid.len() {
        for y in 0..wgrid[x].len() {
            commands.spawn((
                ElementBundle::new(&wgrid[x][y], Element::fullfill()),
                VectorElementRectangle {
                    color: Color::rgb_u8((x * 255/wgrid.len()) as u8, (x * 255/wgrid.len()) as u8, (y * 255/wgrid[x].len()) as u8),
                    corner_radii: Vec4::splat(10.0)
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