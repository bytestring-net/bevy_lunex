use bevy::{prelude::*, window::SystemCursorIcon};
use bevy_lunex::prelude::*;

fn main() -> AppExit {
    App::new()
        .add_plugins((DefaultPlugins, UiLunexPlugins::<0>, UiLunexDebugPlugin::<0, 0>))
        .add_systems(Startup, setup)
        .run()
}

fn setup(
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

            // Spawn a color filled node
            ui.spawn((
                Name::new("My Mesh"),
                // Give it some solid aspect ratio
                UiLayout::solid().size((Ab(1920.0), Ab(1080.0))).pack(),
                // Mark this as UI plane mesh
                UiMeshPlane2d,
                // Give it some material
                MeshMaterial2d(materials.add(Color::srgb(1.0, 0.5, 0.5))),
                // On hover change the cursor to this
                OnHoverSetCursor::new(SystemCursorIcon::Pointer),
            ));
        });
    });
}
