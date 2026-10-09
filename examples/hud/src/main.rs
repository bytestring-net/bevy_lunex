use std::sync::Arc;

use bevy::{anti_alias::fxaa::Fxaa, core_pipeline::oit::OrderIndependentTransparencySettings, prelude::*, window::SystemCursorIcon};
use bevy_lunex::prelude::*;

mod boilerplate;
use boilerplate::*;

fn main() -> AppExit {
    App::new()
        .add_plugins((DefaultPlugins, UiLunexPlugins, UiLunexDebugPlugin::<0, 0>))
        .insert_resource(LoadFonts {
            font_directories: vec!["assets/fonts".to_owned()],
            ..Default::default()
        })
        .add_systems(Startup, setup)

        // This is required for the showcase, not necessary for UI
        .add_plugins(MeshPickingPlugin)
        .add_systems(Update, (ShowcaseCamera::rotate, ShowcaseCamera::zoom))

        .run()
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {
    // Cube
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
        MeshMaterial3d(materials.add(Color::WHITE)),
        Transform::from_rotation(Quat::from_euler(EulerRot::XYX, 1.0, 1.0, 1.0))
    ));

    // Light
    commands.spawn((
        PointLight {
            shadow_maps_enabled: true,
            ..default()
        },
        Transform::from_xyz(4.0, 8.0, 4.0),
    ));

    // Spawn camera
    commands.spawn((
        Camera3d::default(),
        // Required to fix some transparency bugs
        OrderIndependentTransparencySettings::default(), Msaa::Off, Fxaa::default(),
        // Give the camera some controls
        ShowcaseCamera::default(),
    )).with_children(|camera| {

        // Spawn the HUD UI panel
        camera.spawn((
            // Required to mark this as 3D
            UiRoot3d,
            // Use this constructor to init 3D settings
            UiLayoutRoot::new_3d(),
            // Provide default size instead of camera
            Dimension::from((0.5, 0.2)),
            // The location of the UI panel
            Transform::from_xyz(-0.25, 0.0, -0.8)
                .with_rotation(Quat::from_rotation_y(40.0_f32.to_radians())),
        )).with_children(|ui| {

            // Spawn the panel
            ui.spawn((
                Name::new("Panel"),
                // Set the layout of this mesh
                UiLayout::solid().size((818.0, 171.0)).pack(),
                // Provide a material to this mesh
                MeshMaterial3d(materials.add(StandardMaterial {
                    base_color_texture: Some(asset_server.load("panel.png")),
                    alpha_mode: AlphaMode::Blend,
                    unlit: true,
                    ..Default::default()
                })),
                // This component will tell Lunex to reconstruct this mesh as plane on demand
                UiMeshPlane3d,
                // On hover change the cursor to this
                OnHoverSetCursor::new(SystemCursorIcon::Pointer),
            )).with_children(|ui| {

                // Spawn the panel
                ui.spawn((
                    Name::new("Text"),
                    // Set the layout of this mesh
                    UiLayout::window().x(Rl(20.0)).y(Rl(45.0)).anchor(Anchor::CENTER_LEFT).pack(),
                    // This controls the height of the text, so 10% of the parent's node height
                    UiTextSize::from(Rh(20.0)),
                    // Set the starting text value
                    Text3d::new("James 'Left-Hand' Doe"),
                    // Set the text animation
                    TextAnimator::new("James 'Left-Hand' Doe"),      // Note: Changing Text3D after spawn will break mesh picking
                    // Style the 3D text font
                    Text3dStyling {
                        size: 64.0,
                        color: Srgba::rgb_u8(129, 192, 205),
                        align: TextAlign::Center,
                        font: Arc::from("Rajdhani"),
                        weight: Weight::BOLD,
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
                ))
                .observe(|_: On<PointerOut>| info!("Moving out!") )
                .observe(|_: On<PointerOver>| info!("Moving in!") )
                .observe(|_: On<PointerClick>| info!("Click!") );
            });
        });

        // Spawn the HUD UI panel
        camera.spawn((
            // Required to mark this as 3D
            UiRoot3d,
            // Use this constructor to init 3D settings
            UiLayoutRoot::new_3d(),
            // Provide default size instead of camera
            Dimension::from((0.5, 0.2)),
            // The location of the UI panel
            Transform::from_xyz(0.25, 0.0, -0.8)
                .with_rotation(Quat::from_rotation_y(-40.0_f32.to_radians())),
        )).with_children(|ui| {

            // Spawn the panel
            ui.spawn((
                Name::new("Panel"),
                // Set the layout of this mesh
                UiLayout::solid().size((818.0, 171.0)).pack(),
                // Provide a material to this mesh
                MeshMaterial3d(materials.add(StandardMaterial {
                    base_color_texture: Some(asset_server.load("panel.png")),
                    alpha_mode: AlphaMode::Blend,
                    unlit: true,
                    ..Default::default()
                })),
                // This component will tell Lunex to reconstruct this mesh as plane on demand
                UiMeshPlane3d,
                // On hover change the cursor to this
                OnHoverSetCursor::new(SystemCursorIcon::Pointer),
            )).with_children(|ui| {

                // Spawn the panel
                ui.spawn((
                    Name::new("Text"),
                    // Set the layout of this mesh
                    UiLayout::window().x(Rl(20.0)).y(Rl(45.0)).anchor(Anchor::CENTER_LEFT).pack(),
                    // This controls the height of the text, so 10% of the parent's node height
                    UiTextSize::from(Rh(20.0)),
                    // Set the starting text value
                    Text3d::new("James 'Right-Hand' Doe"),
                    // Set the text animation
                    TextAnimator::new("James 'Right-Hand' Doe"),      // Note: Changing Text3D after spawn will break mesh picking
                    // Style the 3D text font
                    Text3dStyling {
                        size: 64.0,
                        color: Srgba::rgb_u8(129, 192, 205),
                        align: TextAlign::Center,
                        font: Arc::from("Rajdhani"),
                        weight: Weight::BOLD,
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
                ))
                .observe(|_: On<PointerOut>| info!("Moving out!") )
                .observe(|_: On<PointerOver>| info!("Moving in!") )
                .observe(|_: On<PointerClick>| info!("Click!") );
            });
        });
    });
}
