use std::borrow::Borrow;

use bevy::prelude::*;
use bevy_lunex::prelude::*;
use bevy_vector_shapes::prelude::*;


fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(Shape2dPlugin::default())
        .add_plugins(LunexUiPlugin)

        .add_systems(Startup, setup)

        .add_systems(Update, dropdown_element_update)

        .add_systems(Update, (
            vector_rectangle_update,
        ).after(element_update))

        .run()
}
fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        Cursor::new(0.0),
        Transform::default(),
    ));
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


    '_Workspace: {
        let workspace = Widget::create(tmp, "workspace", RelativeLayout::new())?;
        commands.spawn((
            ElementBundle::new(workspace.clone(), Element::fullfill()),
            VectorElementRectangle {
                color: Color::rgb_u8(48, 52, 70),
                corner_radii: Vec4::splat(0.0)
            },
        ));

        '_TopPanel: {
            let top_panel = Widget::create(tmp, workspace.end("top_panel"), RelativeLayout {
                relative_2: Vec2::new(100.0, 0.0),
                absolute_2: Vec2::new(0.0, TOPBAR_SIZE),
                ..default()
            })?;
            commands.spawn((
                ElementBundle::new(top_panel.clone(), Element::fullfill()),
                VectorElementRectangle {
                    color: Color::rgb_u8(35, 38, 52),
                    corner_radii: Vec4::new(20.0, 0.0, 0.0, 0.0)
                },
            ));

            '_Grid: {
                let style = TextStyle {
                    font: asset_server.load("Montserrat-Regular.ttf"),
                    font_size: 40.0,
                    color: Color::WHITE,
                };

                let names = textrow!["file", "edit", "preferences", "help"];
                let segment = GridSegment::text_cells(&names, 10.0, 60.0).add_gaps(1.0);
                let (_, wlist) = segment.build_in_solid(tmp, top_panel.end("Grid"), SolidLayout::new().with_horizontal_anchor(-1.0), GridOrientation::Horizontal)?;

                for x in 0..wlist.len() {
                    commands.spawn((
                        ElementBundle::new(&wlist[x], Element::fullfill()),
                        VectorElementRectangle {
                            color: Color::rgb_u8(48, 52, 70),
                            corner_radii: Vec4::splat(4.0)
                        },
                    ));
                    commands.spawn((
                        TextElementBundle::new(&wlist[x], &TextParams::center().with_style(&style).with_height(Some(60.0)), &names[x]),
                        //DropDownElement::new(textrow!["Option1", "Option2"], &style)
                    ));
                    commands.spawn((
                        wlist[x].clone(),
                        DropDownElement::new(textrow!["Option1", "Option2"], &style)
                    ));
                }
            }
        }

        '_SidePanel: {
            let side_panel = Widget::create(tmp, workspace.end("side_panel"), RelativeLayout {
                absolute_1: Vec2::new(0.0, TOPBAR_SIZE),
                absolute_2: Vec2::new(SIDEBAR_SIZE, 0.0),
                relative_2: Vec2::new(0.0, 100.0),
                ..default()
            })?;
            commands.spawn((
                ElementBundle::new(side_panel.clone(), Element::fullfill()),
                VectorElementRectangle {
                    color: Color::rgb_u8(41, 44, 60),
                    corner_radii: Vec4::new(20.0, 0.0, 0.0, 0.0)
                },
            ));
        }

    }

    ui_tree.merge(temporary_tree)?;

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


#[derive(Component)]
pub struct DropDownElement {
    text_style: TextStyle,
    options: Vec<String>,
    select: String,
}
impl DropDownElement {
    pub fn new(options: Vec<String>, text_style: impl Borrow<TextStyle>) -> DropDownElement {
        DropDownElement {
            text_style: text_style.borrow().to_owned(),
            select: options[0].clone(),
            options: options,
        }
    }
    pub fn build_list(&self, commands: &mut Commands, tree: &mut UiTree, widget: &Widget) -> Result<(), LunexError>{
        
        let segment = GridSegment::text_cells(&self.options, 100.0, 60.0);
        let (_, wlist) = segment.build_in_window(tree, widget.end("Droplist"), WindowLayout::new().with_rel(Vec2::new(0.0, 100.0)), GridOrientation::Vertical)?;

        for x in 0..wlist.len() {
            commands.spawn((
                ElementBundle::new(&wlist[x], Element::fullfill()),
                VectorElementRectangle {
                    color: Color::rgb_u8(148, 52, 70),
                    corner_radii: Vec4::splat(4.0)
                },
            ));
            commands.spawn(
                TextElementBundle::new(&wlist[x], &TextParams::centerleft().at(10.0, 50.0).with_style(&self.text_style).with_height(Some(60.0)), &self.options[x])
            );
        }

        Ok(())
    }
}
pub fn dropdown_element_update (mut commands: Commands, mut trees: Query<&mut UiTree>, cursors: Query<&Cursor>, mut query: Query<(&Widget, &DropDownElement)>) {
    for mut tree in &mut trees {
        for (widget, dropdown) in &mut query {
            let mut trigger = false;
            for cursor in &cursors {
                if widget.contains_position(&tree, &cursor.position_world().as_lunex(tree.offset)).unwrap() {
                    trigger = true;
                    break;
                }
            }

            if trigger {
                match widget.fetch_ext(&tree, "Droplist") {
                    Err(..) => {
                        //println!("Building list");
                        dropdown.build_list(&mut commands, &mut tree, widget).unwrap();
                    },
                    Ok (..) => {},
                }
            } else {
                match widget.fetch_ext(&tree, "Droplist") {
                    Err(..) => {},
                    Ok (..) => {
                        //println!("Dropping list");
                        let mut trigger = false;
                        for cursor in &cursors {
                            if widget.contains_position_ext(&tree, "Droplist", &cursor.position_world().as_lunex(tree.offset)).unwrap() {
                                trigger = true;
                                break;
                            }
                        }
                        if trigger == false {
                            widget.remove(&mut tree, "Droplist").unwrap();
                        }
                    },
                }
            }
        }
    }
}