#![allow(clippy::type_complexity)]

use bevy::{asset::RenderAssetUsages, prelude::*, render::{mesh::{Indices, MeshAabb, PrimitiveTopology}, primitives::Aabb}, window::SystemCursorIcon};
use bevy_lunex::{prelude::*, NoLunexPicking};

fn main() -> AppExit {
    App::new()
        .add_plugins((DefaultPlugins, UiLunexPlugins, UiLunexDebugPlugin::<0, 0>))
        .add_plugins(MeshPickingPlugin) // We need to add mesh picking plugin because it is not included by default
        // We need to insert our custom mesh generation system to execute at the right time
        .add_systems(PostUpdate, system_construct_custom_shape_from_dimension.in_set(UiSystems::PostCompute))
        .add_systems(Startup, setup)
        .run()
}

fn setup(
    assets: Res<AssetServer>,
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // Spawn camera
    commands.spawn((
        // This camera will become the source for all UI paired to index 0.
        Camera2d, UiSourceCamera::<0>,

        // Ui nodes start at 0 and move + on the Z axis with each depth layer.
        // This will ensure you will see up to 1000 nested children.
        Transform::from_translation(Vec3::Z * 1000.0),
    ));

    // Spawn the UI Root
    commands.spawn((
        // Name the entity
        Name::new("Root"),

        // Initialize the UI root for 2D
        UiLayoutRoot::new_2d(),

        // Make the UI synchronized with camera viewport size
        UiFetchFromCamera::<0>,
    )).with_children(|ui| {

        // Spawn boundary node
        ui.spawn((
            Name::new("Boundary"),
            // Define the layout
            UiLayout::boundary()
                .pos1(Ab(20.0))
                .pos2(Rl(100.0) - Ab(20.0))
                .pack(),

        // Spawn nested UI nodes
        )).with_children(|ui| {

            // Spawn the text
            ui.spawn((
                Name::new("Text"),
                // Set the layout position of this text
                UiLayout::window().pos(Rl(50.0)).anchor(Anchor::Center).pack(), UiDepth::Add(5.0),
                // This controls the height of the text, so 10% of the parent's node height
                UiTextSize::from(Rh(10.0)),
                // Set the starting text value
                Text2d::new(""),
                // Set the text animation
                TextAnimator::new("Hello 2D UI!"),
                // Style the text font
                TextFont::from_font(assets.load("fonts/Rajdhani-Bold.ttf")).with_font_size(64.0),
            ));

            // Spawn a color filled node
            ui.spawn((
                Name::new("My Mesh"),
                // Give it some solid aspect ratio
                UiLayout::solid().size((Ab(1920.0), Ab(1080.0))).pack(),
                // This will disable the default Lunex 2D picking backend so you can use mesh raycasting backend
                NoLunexPicking,
                // Create your own mesh here for "interaction" bounding box
                CustomUiNodeShape {
                    top_left: Vec2::new(-1.0, -1.0),
                    top_right: Vec2::new(1.0, -1.0),
                    bottom_left: Vec2::new(-0.5, 1.0),
                    bottom_right: Vec2::new(0.5, 1.0),
                },
                // Give it some material
                MeshMaterial2d(materials.add(Color::srgb(0.2, 0.5, 0.8))),
                // On hover change the cursor to this
                OnHoverSetCursor::new(SystemCursorIcon::Pointer),
            ))
            .observe(|_: Trigger<Pointer<Out>>| info!("Moving out!") )
            .observe(|_: Trigger<Pointer<Over>>| info!("Moving in!") )
            .observe(|_: Trigger<Pointer<Click>>| info!("Click!") );
        });
    });
}


#[derive(Component, Reflect, Default, Clone, PartialEq, Debug)]
#[require(Mesh2d)]
struct CustomUiNodeShape {
    bottom_right: Vec2,
    bottom_left: Vec2,
    top_left: Vec2,
    top_right: Vec2,
}


/// This system takes [`Dimension`] data and constructs a custom mesh.
fn system_construct_custom_shape_from_dimension(
    mut query: Query<(&Dimension, &CustomUiNodeShape, &mut Mesh2d, Option<&mut Aabb>), Changed<Dimension>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    for (dimension, shape, mut mesh, aabb_option) in &mut query {

        let positions = vec![
            [dimension.x / 2.0 * shape.bottom_right.x, dimension.y / 2.0 * shape.bottom_right.y, 0.0],
            [dimension.x / 2.0 * shape.bottom_left.x,  dimension.y / 2.0 * shape.bottom_left.y,  0.0],
            [dimension.x / 2.0 * shape.top_left.x,     dimension.y / 2.0 * shape.top_left.y,     0.0],
            [dimension.x / 2.0 * shape.top_right.x,    dimension.y / 2.0 * shape.top_right.y,    0.0],
        ];
        let normals = vec![[0.0, 0.0, 1.0]; 4];
        let uvs = vec![[1.0, 0.0], [0.0, 0.0], [0.0, 1.0], [1.0, 1.0]];
        let indices = Indices::U32(vec![0, 1, 2, 0, 2, 3]);

        let new_mesh = Mesh::new(
            PrimitiveTopology::TriangleList,
            RenderAssetUsages::default(),
        )
        .with_inserted_indices(indices)
        .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, positions)
        .with_inserted_attribute(Mesh::ATTRIBUTE_NORMAL, normals)
        .with_inserted_attribute(Mesh::ATTRIBUTE_UV_0, uvs);

        // Compute AABB for the raycaster
        if let Some(a) = new_mesh.compute_aabb() {
            if let Some(mut aabb) = aabb_option {
                *aabb = a;
            }
        }
        mesh.0 = meshes.add(new_mesh);
    }
}