use bevy::prelude::*;
use bevy::sprite::Anchor;
use bevy_lunex::prelude::*;
use bevy_mod_billboard::prelude::*;
use bevy_mod_billboard::BillboardLockAxis;

mod boilerplate;
use boilerplate::*;

// # ABOUT THIS EXAMPLE
// This is a WIP progress, prototype example, that utilises bevy_mod_billboard for 3D text.
// Everything here is a subject of change, and this implementation will be changed in the
// future. I wish to integrate the crate upstream, but due to me being quite busy, my
// progress is quite slow. In the meantime, this is an example on how to do it yourself.

fn main() {
    App::new()
        .add_plugins((default_plugins(), UiDefaultPlugins, BillboardPlugin))
        .add_systems(Startup, setup)
        .add_systems(Update, (rotate_playercam, zoom_playercam))
        .run();
}

fn setup(
    mut commands: Commands,
    mut material: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {
    // Spawn cursor
    commands.spawn(CursorBundle::default());

    // Spawn camera
    commands.spawn((
        Camera3dBundle::default(),
        PlayerCam {
            orbit: Vec3::new(0.0, 0.0, 0.0),
            distance: 2.0,
            sensitivity: Vec2::splat(0.1),
        }
    ));

    // Spawn the floating Ui panel
    commands.spawn((
        UiTreeBundle::<MainUi> {
            transform: Transform::from_xyz(-0.4, 0.0, 0.0),
            tree: UiTree::new3d("HeaderWidget"),
            ..default()
        },
    )).with_children(|ui| {
        ui.spawn((
            // Link this widget
            UiLink::<MainUi>::path("Root"),

            // The layout that is used when in base state
            UiLayout::window_full().size((818.0, 170.0)).pack::<Base>(),

            // Give the mesh an image
            UiMaterial3dBundle::from_image(&mut material, asset_server.load("header.png")),

            // Make the panel pickable
            PickableBundle::default(),

            // This is required to control our hover animation
            UiAnimator::<Hover>::new().forward_speed(6.0).backward_speed(5.0),

            // This is required for Layout animation
            UiLayoutController::default(),

            // The layout that is used when in hover state
            UiLayout::window_full().x(100.0).size((818.0, 170.0)).pack::<Hover>(),

            // This will change cursor icon on mouse hover
            OnHoverSetCursor::new(CursorIcon::Pointer),
        ));

        ui.spawn((
            // Link this widget
            UiLink::<MainUi>::path("Root/Name"),

            // The layout that is used when in base state
            UiLayout::window().anchor(Anchor::CenterLeft).pos(Rl((20.0, 40.0))).pack::<Base>(),

            // Pop the text further out
            UiDepthBias(20.0),

            // Add the 3D text
            BillboardTextBundle {
                transform: Transform::from_scale(Vec3::splat(0.00085)).looking_at(Vec3::new(0.0, 0.0, 1.0), Vec3::Y),
                text: Text::from_sections([
                    TextSection {
                        value: "SQ. JANE KELLY".to_string(),
                        style: TextStyle {
                            font_size: 60.0,
                            font: asset_server.load("rajdhani/Rajdhani-SemiBold.ttf"),
                            color: Color::Srgba(Srgba::rgb_u8(129, 192, 205)),
                        },
                    },
                ])
                .with_justify(JustifyText::Left),
                text_anchor: Anchor::CenterLeft,
                ..default()
            },

            // Lock the billboard automatic rotation
            BillboardLockAxis {
                rotation: true,
                ..default()
            },
        ));

        ui.spawn((
            // Link this widget
            UiLink::<MainUi>::path("Root/Missions"),

            // The layout that is used when in base state
            UiLayout::window().anchor(Anchor::CenterLeft).pos(Rl((35.0, 77.0))).pack::<Base>(),

            // Pop the text further out
            UiDepthBias(20.0),

            // Add the 3D text
            BillboardTextBundle {
                transform: Transform::from_scale(Vec3::splat(0.00085)).looking_at(Vec3::new(0.0, 0.0, 1.0), Vec3::Y),
                text: Text::from_sections([
                    TextSection {
                        value: "6".to_string(),
                        style: TextStyle {
                            font_size: 60.0,
                            font: asset_server.load("rajdhani/Rajdhani-SemiBold.ttf"),
                            color: Color::Srgba(Srgba::rgb_u8(129, 192, 205)),
                        },
                    },
                ])
                .with_justify(JustifyText::Left),
                text_anchor: Anchor::CenterLeft,
                ..default()
            },

            // Lock the billboard automatic rotation
            BillboardLockAxis {
                rotation: true,
                ..default()
            },
        ));

        ui.spawn((
            // Link this widget
            UiLink::<MainUi>::path("Root/Kills"),

            // The layout that is used when in base state
            UiLayout::window().anchor(Anchor::CenterLeft).pos(Rl((55.0, 77.0))).pack::<Base>(),

            // Pop the text further out
            UiDepthBias(20.0),

            // Add the 3D text
            BillboardTextBundle {
                transform: Transform::from_scale(Vec3::splat(0.00085)).looking_at(Vec3::new(0.0, 0.0, 1.0), Vec3::Y),
                text: Text::from_sections([
                    TextSection {
                        value: "17".to_string(),
                        style: TextStyle {
                            font_size: 60.0,
                            font: asset_server.load("rajdhani/Rajdhani-SemiBold.ttf"),
                            color: Color::Srgba(Srgba::rgb_u8(129, 192, 205)),
                        },
                    },
                ])
                .with_justify(JustifyText::Left),
                text_anchor: Anchor::CenterLeft,
                ..default()
            },

            // Lock the billboard automatic rotation
            BillboardLockAxis {
                rotation: true,
                ..default()
            },
        ));

        ui.spawn((
            // Link this widget
            UiLink::<MainUi>::path("Root/Status"),

            // The layout that is used when in base state
            UiLayout::window().anchor(Anchor::CenterLeft).pos(Rl((74.0, 77.0))).pack::<Base>(),

            // Pop the text further out
            UiDepthBias(20.0),

            // Add the 3D text
            BillboardTextBundle {
                transform: Transform::from_scale(Vec3::splat(0.0007)).looking_at(Vec3::new(0.0, 0.0, 1.0), Vec3::Y),
                text: Text::from_sections([
                    TextSection {
                        value: "ON MISSION".to_string(),
                        style: TextStyle {
                            font_size: 60.0,
                            font: asset_server.load("rajdhani/Rajdhani-Regular.ttf"),
                            color: Color::Srgba(Srgba::rgb_u8(129, 192, 205)),
                        },
                    },
                ])
                .with_justify(JustifyText::Left),
                text_anchor: Anchor::CenterLeft,
                ..default()
            },

            // Lock the billboard automatic rotation
            BillboardLockAxis {
                rotation: true,
                ..default()
            },
        ));
    });
}
