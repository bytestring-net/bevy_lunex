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
fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
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
    println!("{}", ui_tree.generate_map_debug());
    commands.spawn (ui_tree);
}

pub fn build_interface (commands: &mut Commands, asset_server: &Res<AssetServer>, ui_tree: &mut UiTree) -> Result<(), LunexError> {

    const TOPBAR_SIZE: f32 = 25.0;
    const SIDEBAR_SIZE: f32 = 70.0;

    let mut temporary_tree = UiTree::new("tmp");
    let tmp = &mut temporary_tree;

    
    let workspace = Widget::create(tmp, "workspace", RelativeLayout::new())?;

    let top_panel = Widget::create(tmp, workspace.end("top_panel"), RelativeLayout {
        relative_2: Vec2::new(100.0, 0.0),
        absolute_2: Vec2::new(0.0, TOPBAR_SIZE),
        ..default()
    })?;
    let side_panel = Widget::create(tmp, workspace.end("side_panel"), RelativeLayout {
        absolute_1: Vec2::new(0.0, TOPBAR_SIZE),
        absolute_2: Vec2::new(SIDEBAR_SIZE, 0.0),
        relative_2: Vec2::new(0.0, 100.0),
        ..default()
    })?;



    let style = TextStyle {
        font: asset_server.load("Montserrat-Regular.ttf"),
        font_size: 40.0,
        color: Color::WHITE,
    };

    let names = textrow!["file", "edit", "preferences", "help"];
    let segment = GridSegment::text_cells(&names, 10.0, 60.0);
    let (_, wlist) = segment.build_in_solid(tmp, top_panel.end("Grid"), SolidLayout::new().with_horizontal_anchor(-1.0), GridOrientation::Horizontal)?;


    ui_tree.merge(temporary_tree)?;

    
    '_Fills: {
        commands.spawn((
            ElementBundle::new(workspace.clone(), Element::fullfill()),
            VectorElementRectangle {
                color: Color::rgb_u8(48, 52, 70),
                corner_radii: Vec4::splat(0.0)
            },
        ));
        commands.spawn((
            ElementBundle::new(top_panel.clone(), Element::fullfill()),
            VectorElementRectangle {
                color: Color::rgb_u8(35, 38, 52),
                corner_radii: Vec4::new(20.0, 0.0, 0.0, 0.0)
            },
        ));
        commands.spawn((
            ElementBundle::new(side_panel.clone(), Element::fullfill()),
            VectorElementRectangle {
                color: Color::rgb_u8(41, 44, 60),
                corner_radii: Vec4::new(20.0, 0.0, 0.0, 0.0)
            },
        ));
        for x in 0..wlist.len() {
            commands.spawn((
                ElementBundle::new(&wlist[x], Element::fullfill()),
                VectorElementRectangle {
                    color: Color::rgb_u8(48, 52, 70),
                    corner_radii: Vec4::splat(4.0)
                },
            ));
            commands.spawn(
                TextElementBundle::new(&wlist[x], &TextParams::center().with_style(&style).with_height(Some(60.0)), &names[x])
            );
        }
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


pub struct DropDownElement {
    text_style: TextStyle,
    options: Vec<String>,
    select: String,
}
impl DropDownElement {
    pub fn new(options: Vec<String>, text_style: TextStyle) -> DropDownElement {
        DropDownElement {
            text_style,
            select: options[0].clone(),
            options: options,
        }
    }
    pub fn build_list(&self, commands: &mut Commands, tree: &mut UiTree, widget: &Widget) -> Result<(), LunexError>{
        let row = textrow!["Option 1", "Option 2", "Option 3"];

        let segment = GridSegment::splat_cells(GridCell::new(), 5);

        let (_, wlist) = segment.build_in_window(tree, widget.end(""), WindowLayout::new().with_rel(Vec2::new(0.0, 100.0)), GridOrientation::Vertical)?;


        for x in 0..wlist.len() {
            commands.spawn((
                ElementBundle::new(&wlist[x], Element::fullfill()),
                VectorElementRectangle {
                    color: Color::rgb_u8(48, 52, 70),
                    corner_radii: Vec4::splat(4.0)
                },
            ));
            commands.spawn(
                TextElementBundle::new(&wlist[x], &TextParams::center().with_style(&self.text_style).with_height(Some(60.0)), &row[x])
            );
        }

        Ok(())
    }
}