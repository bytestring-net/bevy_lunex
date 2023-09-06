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
            vector_fill_update,
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

    const TOPBAR_SIZE: f32 = 35.0;
    const SIDEBAR_SIZE: f32 = 70.0;

    let mut temporary_tree = UiTree::new("tmp");
    let tmp = &mut temporary_tree;

    
    let workspace = Widget::create(tmp, "workspace", RelativeLayout::new().pack())?;

    let top_panel = Widget::create(tmp, &workspace.end("top_panel"), RelativeLayout {
        relative_2: Vec2::new(100.0, 0.0),
        absolute_2: Vec2::new(0.0, TOPBAR_SIZE),
        ..default()
    }.pack())?;
    let side_panel = Widget::create(tmp, &workspace.end("side_panel"), RelativeLayout {
        absolute_1: Vec2::new(0.0, TOPBAR_SIZE),
        absolute_2: Vec2::new(SIDEBAR_SIZE, 0.0),
        relative_2: Vec2::new(0.0, 100.0),
        ..default()
    }.pack())?;


    let names = textgrid![["file"],["edit"],["preferences"],["help"]];
    let grid = GridParams::new(&names).with_anchor(bevy::sprite::Anchor::CenterLeft).with_width(100.0).with_height(20.0).with_width_gap(10.0);
    let wgrid = grid_generate_solid(tmp, &top_panel.end("navbar"), &grid)?;

    /*commands.spawn((
        ElementBundle::new(wgrid.clone(), Element::fullfill()),
        VectorElementColorFill (Color::rgb(200./255., 200./255., 200./255.)),
    ));*/

    //# Loop over grid of widgets in 'nameless'
    for x in 0..names.len() {
        for y in 0..names[0].len() {

            //# Spawn image for widgets in 'nameless'
            let widget = Widget::new(&wgrid.end(&names[x][y]));
            /*commands.spawn((
                ElementBundle::new(widget.clone(), Element::fullfill()),
                VectorElementColorFill (Color::rgb(200./255., 200./255., 200./255.)),
            ));*/

            commands.spawn(
                TextElementBundle::new(widget.clone(), &TextParams::default(), &names[x][y])
            );
        }
    }

    ui_tree.merge(temporary_tree)?;

    '_Fills: {

        commands.spawn((
            ElementBundle::new(workspace.clone(), Element::fullfill()),
            VectorElementColorFill (Color::rgb(48./255., 52./255., 70./255.)),
        ));
        commands.spawn((
            ElementBundle::new(top_panel.clone(), Element::fullfill()),
            VectorElementColorFill (Color::rgb(35./255., 38./255., 52./255.)),
        ));
        commands.spawn((
            ElementBundle::new(side_panel.clone(), Element::fullfill()),
            VectorElementColorFill (Color::rgb(41./255., 44./255., 60./255.)),
        ));

    }



    Ok(())
}

// DEFINE STYLE

#[derive(Component)]
pub struct VectorElementColorFill (Color);
pub fn vector_fill_update (mut painter: ShapePainter, query: Query<(&Transform, &VectorElementColorFill)>) {
    for (transform, color) in &query {

        painter.set_translation(transform.translation);
        painter.set_scale(Vec3::splat(1.0));

        let ww = transform.scale.x;
        let hh = transform.scale.y;

        painter.color = color.0;
        painter.thickness = 1.0;
        painter.rect(Vec2::new(ww, hh));
    }
}
