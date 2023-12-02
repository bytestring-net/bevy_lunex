use std::borrow::Borrow;
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
        .add_plugins(LunexUiDebugPlugin2D::<D>::new())

        .add_systems(Startup, setup)
        .add_systems(Update, dropdown_element_update::<D>)

        .add_systems(Update, (
            vector_rectangle_update
        ).after(LunexUiSystemSet2D))

        .run()
}
fn setup(mut commands: Commands, asset_server: Res<AssetServer>, window: Query<Entity, (With<Window>, With<PrimaryWindow>)>) {
    // Spawn cursor
    commands.spawn((
        Cursor::new(),
        Transform::default(),
        Visibility::default()
    ));

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
                let segment = GridSegment::text_cells(&names, 100.0, 60.0).add_gaps(1.0);
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

#[derive(Component)]
struct DropDownElement {
    text_style: TextStyle,
    options: Vec<String>,
    _selected: (String, usize),
}
impl DropDownElement {
    fn new(options: Vec<String>, text_style: impl Borrow<TextStyle>) -> DropDownElement {
        DropDownElement {
            text_style: text_style.borrow().to_owned(),
            _selected: (options[0].clone(), 0),
            options: options,
        }
    }
    fn build_list<T:Default>(&self, commands: &mut Commands, tree: &mut UiTree<T>, widget: &Widget) -> Result<(), LunexError>{
        
        let segment = GridSegment::text_cells(&self.options, 50.0, 60.0).add_gaps(1.0);
        let (_, wlist) = segment.build_in_window_absolute(tree, widget.end("Droplist"), WindowLayout::empty().with_rel(Vec2::new(0.0, 100.0)), GridOrientation::Vertical)?;

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
fn dropdown_element_update<T:Default+Component>(mut commands: Commands, mut trees: Query<&mut UiTree<T>>, cursors: Query<&Cursor>, mut query: Query<(&Widget, &DropDownElement)>) {
    for mut tree in &mut trees {
        for (widget, dropdown) in &mut query {
            let mut trigger = false;
            for cursor in &cursors {
                if widget.contains_position(&tree, &cursor.location_world().invert_y()).unwrap() {
                    trigger = true;
                    break;
                }
            }

            if trigger {
                match widget.fetch_ext(&tree, "Droplist") {
                    Err(..) => {
                        dropdown.build_list(&mut commands, &mut tree, widget).unwrap();
                    },
                    Ok (..) => {},
                }
            } else {
                match widget.fetch_ext(&tree, "Droplist") {
                    Err(..) => {},
                    Ok (..) => {
                        for cursor in &cursors {
                            if widget.contains_position_ext(&tree, "Droplist", &cursor.location_world().invert_y()).unwrap() {
                                trigger = true;
                                break;
                            }
                        }
                        if trigger == false {
                            widget.drop_branch(&mut tree, "Droplist").unwrap();
                        }
                    },
                }
            }
        }
    }
}