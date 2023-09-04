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

    let mut temporary_tree = UiTree::new("tmp");
    let tmp = &mut temporary_tree;

    
    let top_panel = Widget::create(tmp, "top_panel", RelativeLayout::new().with_rel_2(Vec2::new(100.0, 7.0)).pack())?;
    let side_panel = Widget::create(tmp, "side_panel", RelativeLayout::new().with_rel_1(Vec2::new(0.0, 7.0)).with_rel_2(Vec2::new(10.0, 100.0)).pack())?;

    ui_tree.merge(temporary_tree)?;


    commands.spawn((
        ElementBundle::new(top_panel.clone(), Element::fullfill().with_depth(100.0)),
        VectorElementColorFill (Color::rgb(30./255., 31./255., 34./255.)),
    ));

    commands.spawn((
        ElementBundle::new(side_panel.clone(), Element::fullfill().with_depth(100.0)),
        VectorElementColorFill (Color::rgb(43./255., 45./255., 49./255.)),
    ));


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
