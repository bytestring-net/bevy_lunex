#![allow(clippy::type_complexity)]

use bevy::prelude::*;
use bevy::window::PrimaryWindow;


#[derive(Component)]
#[require(Visibility, Transform, Dimension)]
pub struct UiLayoutRoot;


#[derive(Component)]
#[require(Visibility, Transform, Dimension)]
pub struct UiLayout {

}
impl UiLayout {
    pub fn window() -> Self {
        UiLayout {}
    }
    pub fn solid() -> Self {
        UiLayout {}
    }
    pub fn boundary() -> Self {
        UiLayout {}
    }
}


#[derive(Component, Deref, DerefMut, Default, Clone, PartialEq, Debug)]
pub struct Dimension(pub Vec2);




#[derive(Component, Clone, PartialEq, Debug)]
pub struct UiFetchFromCamera<const INDEX: usize>;

#[derive(Component, Clone, PartialEq, Debug)]
pub struct UiSourceCamera<const INDEX: usize>;


/// This system takes [`Camera`] viewport data and pipes them into querried [`Dimension`] + [`UiLayoutRoot`] + [`UiFetchFromCamera`].
pub fn fetch_dimension_from_camera<const INDEX: usize>(
    src_query: Query<(&Camera, Option<&OrthographicProjection>), (With<UiSourceCamera<INDEX>>, Changed<Camera>)>,
    mut dst_query: Query<&mut Dimension, (With<UiLayoutRoot>, With<UiFetchFromCamera<INDEX>>)>,
) {
    // Check if we have a camera dimension input
    if src_query.is_empty() { return; }
    let Ok((camera, projection_option)) = src_query.get_single() else {
        warn_once!("Multiple UiSourceCamera<{INDEX}> exist at once! Ignoring all camera inputs to avoid unexpected behavior!");
        return;
    };

    // Pipe the camera viewport size
    if let Some(cam_size) = camera.physical_viewport_size() {
        for mut size in &mut dst_query {
            **size = Vec2::from((cam_size.x as f32, cam_size.y as f32)) * if let Some(p) = projection_option { p.scale } else { 1.0 };
        }
    }
}

/// This system takes [`Camera`] viewport data and pipes them into querried [`Transform`] + [`UiLayoutRoot`] + [`UiFetchFromCamera`].
pub fn fetch_transform_from_camera<const INDEX: usize>(
    src_query: Query<(&Camera, Option<&OrthographicProjection>), (With<UiSourceCamera<INDEX>>, Changed<Camera>)>,
    mut dst_query: Query<&mut Transform, (With<UiLayoutRoot>, With<UiFetchFromCamera<INDEX>>)>,
    window: Query<&Window, With<PrimaryWindow>>,
) {
    // Check if we have a camera dimension input
    if src_query.is_empty() { return; }
    let Ok((camera, projection_option)) = src_query.get_single() else {
        warn_once!("Multiple UiSourceCamera<{INDEX}> exist at once! Ignoring all camera inputs to avoid unexpected behavior!");
        return;
    };

    // Get the resolution scale of a window
    let res_scale = if let Ok(window) = window.get_single() { window.resolution.scale_factor() } else { 1.0 };

    // Pipe the camera location
    if let Some(cam_size) = camera.physical_viewport_size() {
        for mut transform in &mut dst_query {
            let scale = if let Some(p) = projection_option { p.scale } else { 1.0 };
            transform.translation.x = (cam_size.x as f32 /-2.0 / res_scale) * scale;
            transform.translation.y = (cam_size.y as f32 / 2.0 / res_scale) * scale;
        }
    }
}

/// This system draws the outlines of [`UiLayout`] and [`UiLayoutRoot`] as gizmos.
pub fn debug_draw_gizmo<G:GizmoConfigGroup>(
    query: Query<(&GlobalTransform, &Dimension), Or<(With<UiLayout>, With<UiLayoutRoot>)>>,
    mut gizmos: Gizmos<G>
) {
    for (transform, dimension) in &query {

        // Align the gizmo to top left corner
        let position = transform.translation();
        let position = position + transform.right() * dimension.x / 2.0;
        let position = position + transform.down() * dimension.y / 2.0;

        // Draw the gizmo outline
        gizmos.rect(
            Isometry3d::from(position),
            **dimension,
            Color::linear_rgb(0.0, 1.0, 0.0),
        );
    }
}


pub struct UiLunexPlugin;
impl Plugin for UiLunexPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (
            fetch_dimension_from_camera::<0>,
            fetch_dimension_from_camera::<1>,
            fetch_dimension_from_camera::<2>,
            fetch_dimension_from_camera::<3>,
        ));

        app.add_systems(Update, (
            fetch_transform_from_camera::<0>,
            fetch_transform_from_camera::<1>,
            fetch_transform_from_camera::<2>,
            fetch_transform_from_camera::<3>,
        ));

        app.add_systems(Update, (
            debug_draw_gizmo::<DefaultGizmoConfigGroup>,
        ));
    }
}

