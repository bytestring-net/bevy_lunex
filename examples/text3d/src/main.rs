use bevy::{prelude::*, window::SystemCursorIcon};
use bevy_lunex::*;
use bevy_rich_text3d::*;

mod boilerplate;
use boilerplate::*;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, UiLunexPlugin, UiLunexDebugPlugin::<0, 0>))
        .add_systems(Startup, setup)

        // Add plugin implementing 3rd-party hooks for Lunex
        .add_plugins(UiLunexText3dPlugin)

        // This is required for the showcase, not necessary UI
        .add_plugins(MeshPickingPlugin)
        .add_systems(Update, (ShowcaseCamera::rotate, ShowcaseCamera::zoom))

        .run();
}

fn setup(
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
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
                UiLayout::window().pos(Rl(50.0)).anchor(Anchor::Center).pack(),
                // This controls the height of the text, so 10% of the parent's node height
                UiTextSize::from(Rh(10.0)),
                // Set the text value
                Text3d::new("Hello UI!"),
                // Style the 3D text
                Text3dStyling {
                    size: 64.0,
                    color: Srgba::new(1., 1., 1., 1.),
                    align: TextAlign::Center,
                    ..Default::default()
                },
                // Provide a material to this mesh
                MeshMaterial3d(materials.add(
                    StandardMaterial {
                        base_color_texture: Some(TextAtlas::DEFAULT_IMAGE),
                        alpha_mode: AlphaMode::Blend,
                        unlit: true,
                        ..Default::default()
                    }
                )),
                // Requires an empty mesh
                Mesh3d::default(),
                // On hover change the cursor to this
                OnHoverSetCursor::new(SystemCursorIcon::Pointer),
            ));
        });

    }
}


/// The 3D Text plugin is not officially implemented in Lunex yet.
/// So we need to add support to in manually.
///
/// This showcases how to hook 3rd-party libraries to Lunex calculation process.
pub struct UiLunexText3dPlugin;
impl Plugin for UiLunexText3dPlugin {
    fn build(&self, app: &mut App) {

        // Register the 3rd party plugin
        app.add_plugins(Text3dPlugin {
            load_system_fonts: true,
            load_font_directories: vec!["assets/fonts".to_owned()],
            ..Default::default()
        });

        // Add system converting text size to layout
        app.add_systems(PostUpdate,
            system_text_3d_size_to_layout
                .after(bevy_rich_text3d::Text3dSet)
                .in_set(UiSystems::PreCompute)
        );

        // Add system scaling the transform based on dimension after calculation
        app.add_systems(PostUpdate,
            system_text_3d_size_from_dimension
                .in_set(UiSystems::PostCompute)
        );
    }
}

/// This system takes updated [`Text3dDimensionOut`] data and overwrites coresponding [`UiLayout`] data to match the text size.
pub fn system_text_3d_size_to_layout(
    mut commands: Commands,
    mut query: Query<(&mut UiLayout, &Text3dDimensionOut, &UiTextSize), Changed<Text3dDimensionOut>>,
) {
    for (mut layout, text_info, text_size) in &mut query {
        // Wait for text to render
        if text_info.dimension.y == 0.0 {
            commands.trigger(RecomputeUiLayout);
            continue;
        }

        // Create the text layout
        match layout.layouts.get_mut(&UiBase::id()).expect("UiBase state not found for Text") {
            UiLayoutType::Window(window) => {
                window.set_height(**text_size);
                window.set_width(**text_size * (text_info.dimension.x / text_info.dimension.y));
            },
            UiLayoutType::Solid(solid) => {
                solid.set_size(Ab(text_info.dimension));
            },
            _ => {},
        }
    }
}

/// This system takes [`Text3dDimensionOut`] data and pipes them into querried [`Transform`] scale.
pub fn system_text_3d_size_from_dimension(
    mut commands: Commands,
    mut query: Query<(&mut Transform, &Dimension, &Text3dDimensionOut), Changed<Dimension>>,
) {
    for (mut transform, dimension, text_info) in &mut query {
        // Wait for text to render
        if text_info.dimension.y == 0.0 {
            commands.trigger(RecomputeUiLayout);
            continue;
        }

        // Scale the text
        let scale = dimension.x / text_info.dimension.x;
        transform.scale.x = scale;
        transform.scale.y = scale;
    }
}
