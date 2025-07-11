#![allow(clippy::type_complexity)]

use bevy::{prelude::*, render::{camera::RenderTarget, view::RenderLayers}};
use bevy_lunex::prelude::*;

fn main() -> AppExit {
    App::new()
        .add_plugins((
            DefaultPlugins.build().set(ImagePlugin::default_nearest()),
            UiLunexPlugins, UiLunexDebugPlugin::<0, 0>,
        ))
        .add_systems(Startup, (spawn_scene, spawn_cameras))
        .run()
}


/// This system will spawn the dual cameras
fn spawn_cameras(
    mut commands: Commands,
    mut images: ResMut<Assets<Image>>,
) {
    // Create the virtual texture with the given size
    let canvas = images.add(virtual_texture(480, 270));

    // Spawn the composition camera
    commands.spawn((
        Camera2d,
        // Configure it as UI source
        UiSourceCamera::<0>,
        // Set the camera location to capture spawned sprites
        Transform::from_translation(Vec3::Z * 1000.0),
        // Set the render layers to only see the canvas
        RenderLayers::from_layers(&[1]),
    ));

    // Compose the secondary canvas camera infront of composition camera
    commands.spawn((
        Name::new("Canvas composition"),
        UiLayoutRoot::new_2d(),
        UiFetchFromCamera::<0>,
    )).with_children(|ui| {

        // Plane with 3D camera canvas, 480:270 aspect ratio
        ui.spawn((
            Name::new("Canvas"),
            UiLayout::solid().size((480.0, 270.0)).scaling(Scaling::Fit).pack(),
            Sprite::from_image(canvas.clone()),
            RenderLayers::from_layers(&[1]),
        ));
    });

    // Spawn the canvas camera
    commands.spawn((
        Camera3d::default(),
        // Configure the camera
        Camera {
            target: RenderTarget::Image(canvas.into()),
            clear_color: ClearColorConfig::Custom(Color::srgba(0.0, 0.0, 0.0, 0.5)),
            order: -1,
            ..Default::default()
        },
        // Disable AntiAliasing for that raw pixelated look
        Msaa::Off,
        // Set the camera location to capture spawned scene
        Transform::from_xyz(-2.5, 4.5, 9.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
}

// Spawn scene
fn spawn_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // circular base
    commands.spawn((
        Mesh3d(meshes.add(Circle::new(4.0))),
        MeshMaterial3d(materials.add(Color::WHITE)),
        Transform::from_rotation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2)),
    ));
    // cube
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
        MeshMaterial3d(materials.add(Color::srgb_u8(124, 144, 255))),
        Transform::from_xyz(0.0, 0.5, 0.0),
    ));
    // light
    commands.spawn((
        PointLight {
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(4.0, 8.0, 4.0),
    ));
}

fn virtual_texture(width: u32, height: u32) -> Image {
    use bevy::render::render_resource::{Extent3d, TextureDimension, TextureFormat, TextureUsages};
    use bevy::asset::RenderAssetUsages;

    let mut image = Image::new_fill(
        Extent3d {
            width,
            height,
            ..Default::default()
        },
        TextureDimension::D2,
        &[0, 0, 0, 0],
        TextureFormat::Bgra8UnormSrgb,
        RenderAssetUsages::default(),
    );
    image.texture_descriptor.usage = TextureUsages::TEXTURE_BINDING | TextureUsages::COPY_DST | TextureUsages::RENDER_ATTACHMENT;
    image
}