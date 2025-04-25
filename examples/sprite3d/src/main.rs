use bevy::{prelude::*, window::SystemCursorIcon};
use bevy_lunex::prelude::*;

mod boilerplate;
use boilerplate::*;

fn main() -> AppExit {
    App::new()
        .add_plugins((DefaultPlugins, UiLunexPlugins::<0>, UiLunexDebugPlugin::<0, 0>))
        .add_systems(Startup, setup)

        // This is required for the showcase, not necessary for UI
        .add_plugins(MeshPickingPlugin)
        .add_systems(Update, (ShowcaseCamera::rotate, ShowcaseCamera::zoom))

        .run()
}

fn setup(
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {
    // Spawn camera
    commands.spawn((
        Camera3d::default(),
        // Give the camera some controls
        ShowcaseCamera::default(),
    ));

    // Spawn it 3 times
    for x in [-1, 0, 1] {

        // Spawn the floating UI panel
        commands.spawn((
            // Required to mark this as 3D
            UiRoot3d,
            // Use this constructor to init 3D settings
            UiLayoutRoot::new_3d(),
            // Provide default size instead of camera
            Dimension::from((0.818, 0.965)),
            // The location of the UI panel
            Transform::from_translation(Vec3::Z * (0.3 * x as f32)),
        )).with_children(|ui| {

            // Spawn the panel
            ui.spawn((
                Name::new("Panel"),
                // Set the layout of this mesh
                UiLayout::window().full().pack(),
                // Provide a material to this mesh
                MeshMaterial3d(materials.add(StandardMaterial {
                    base_color_texture: Some(asset_server.load("panel.png")),
                    alpha_mode: AlphaMode::Blend,
                    unlit: true,
                    ..default()
                })),
                // This component will tell Lunex to reconstruct this mesh as plane on demand
                UiMeshPlane3d,
                // On hover change the cursor to this
                OnHoverSetCursor::new(SystemCursorIcon::Pointer),
            ));
        });

    }
}
