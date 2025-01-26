#![allow(clippy::type_complexity)]

use bevy::prelude::*;


#[derive(Component)]
#[require(Visibility, Transform, Dimension)]
pub struct UiLayoutRoot;

#[derive(Component)]
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



#[derive(Component, Deref, DerefMut, Clone, PartialEq, Debug)]
pub struct UiSource(pub Entity);


/// This system takes [`Camera`] data and overwrites querried [`Dimension`] + [`SourceFromCamera`].
/// It is mainly used to pipe [`Camera`] data into [`UiRoot`] for root node computation.
pub fn fetch_size_from_camera(
    src_query: Query<(&Camera, Option<&OrthographicProjection>), Changed<Camera>>,
    mut dst_query: Query<(&mut Dimension, &UiSource), With<UiLayoutRoot>>
) {
    if src_query.is_empty() { return; }

    for (mut size, source) in &mut dst_query {
        // Retrieve targetted camera
        if let Ok((camera, projection_option)) = src_query.get(**source) {
            // Pipe the viewport size from camera
            if let Some(cam_size) = camera.physical_viewport_size() {
                **size = Vec2::from((cam_size.x as f32, cam_size.y as f32)) * if let Some(p) = projection_option { p.scale } else { 1.0 };
                println!("{:?}", size);
            }
        }
    }
}





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
            println!("{:?}", size);
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
            //fetch_transform_from_camera::<1>,
            //fetch_transform_from_camera::<2>,
            //fetch_transform_from_camera::<3>,
        ));
    }
}

