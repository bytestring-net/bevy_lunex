use bevy::prelude::*;
use bevy_lunex::prelude::*;

mod boilerplate;
use boilerplate::*;


fn main() {
    App::new()
        .add_plugins((default_plugins(), UiPlugin))
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

    // Spawn it 3 times
    for x in [-1, 0, 1] {

        // Spawn the floating Ui panel
        commands.spawn((
            UiTreeBundle::<MainUi> {
                transform: Transform::from_xyz(-0.4, 0.3, 0.0 + (0.3 * x as f32)),
                tree: UiTree::new3d("PanelWidget"),
                ..default()
            },
        )).with_children(|ui| {
            ui.spawn((
                // Link this widget
                UiLink::<MainUi>::path("Root"),

                // The layout that is used when in base state
                UiLayout::window_full().size((818.0, 965.0)).pack::<Base>(),

                // Give the mesh an image
                UiMaterial3dBundle::from_transparent_image(&mut material, asset_server.load("panel.png")),

                // Make the panel pickable
                PickableBundle::default(),

                // This is required to control our hover animation
                UiAnimator::<Hover>::new().forward_speed(6.0).backward_speed(5.0),

                // This is required for Layout animation
                UiLayoutController::default(),

                // The layout that is used when in hover state
                UiLayout::window_full().x(100.0).size((818.0, 965.0)).pack::<Hover>(),

                // This will change cursor icon on mouse hover
                OnHoverSetCursor::new(CursorIcon::Pointer),
            ));
        }); 
    }
}
