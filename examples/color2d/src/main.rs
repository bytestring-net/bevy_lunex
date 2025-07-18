use bevy::{prelude::*, window::SystemCursorIcon};
use bevy_lunex::prelude::*;

fn main() -> AppExit {
    App::new()
        .add_plugins((DefaultPlugins, UiLunexPlugins, UiLunexDebugPlugin::<0, 0>))
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
                // Mark this as UI plane mesh
                UiMeshPlane2d,
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