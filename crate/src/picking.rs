use bevy_math::FloatExt;
use bevy_window::PrimaryWindow;
use bevy_picking::backend::prelude::*;
use bevy_picking::{backend::PointerHits, Pickable};
use bevy_camera::visibility::{RenderLayers, DEFAULT_LAYERS};

use crate::*;


// #===============#
// #=== BACKEND ===#

/// Adds picking support for Lunex.
#[derive(Clone)]
pub struct UiLunexPickingPlugin;
impl Plugin for UiLunexPickingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreUpdate, lunex_2d_picking.in_set(PickingSystems::Backend));
    }
}

/// This component disables the Lunex picking backend for this entity.
/// Use this only if you want to use a different or custom picking
/// bakckend. To disable picking entirely, use [`Pickable::IGNORE`].
#[derive(Component)]
pub struct NoLunexPicking;

/// Checks if any Dimension entities are under a pointer
fn lunex_2d_picking(
    pointers: Query<(&PointerId, &PointerLocation)>,
    cameras: Query<(
        Entity,
        &Camera,
        &GlobalTransform,
        &Projection,
        Option<&RenderLayers>,
    )>,
    primary_window: Query<Entity, With<PrimaryWindow>>,
    lunex_query: Query<(
        Entity,
        &Dimension,
        &GlobalTransform,
        Option<&Pickable>,
        &ViewVisibility,
        Option<&RenderLayers>,
    ), Without<NoLunexPicking>>,
    mut output: MessageWriter<PointerHits>,
) {
    let mut sorted_nodes: Vec<_> = lunex_query
        .iter()
        .filter_map(|(entity, dimension, transform, pickable, vis, render_layers)| {
            if !transform.affine().is_nan() && vis.get() {
                Some((entity, dimension, transform, pickable, render_layers))
            } else {
                None
            }
        }).collect();

    // radsort is a stable radix sort that performed better than `slice::sort_by_key`
    radsort::sort_by_key(&mut sorted_nodes, |(_, _, transform, _, _)| {
        -transform.translation().z
    });

    let primary_window = primary_window.single().ok();

    for (pointer, location) in pointers.iter().filter_map(|(pointer, pointer_location)| {
        pointer_location.location().map(|loc| (pointer, loc))
    }) {
        let mut blocked = false;

        // Find the highest-order active camera that matches this pointer's target
        let Some((cam_entity, camera, cam_transform, Projection::Orthographic(cam_ortho), cam_render_layers)) =
            cameras
                .iter()
                .filter(|(_, camera, _, _, _)| {
                    camera.is_active
                        && camera
                            .target
                            .normalize(primary_window)
                            .is_some_and(|x| x == location.target)
                })
                .max_by_key(|(_, camera, _, _, _)| camera.order)
        else {
            continue;
        };

        let cam_layers = cam_render_layers.unwrap_or(DEFAULT_LAYERS);

        let viewport_pos = camera
            .logical_viewport_rect()
            .map(|v| v.min)
            .unwrap_or_default();
        let pos_in_viewport = location.position - viewport_pos;

        let Ok(cursor_ray_world) = camera.viewport_to_world(cam_transform, pos_in_viewport) else {
            continue;
        };
        let cursor_ray_len = cam_ortho.far - cam_ortho.near;
        let cursor_ray_end = cursor_ray_world.origin + cursor_ray_world.direction * cursor_ray_len;

        let picks: Vec<(Entity, HitData)> = sorted_nodes
            .iter()
            .copied()
            .filter_map(|(entity, dimension, node_transform, pickable, entity_render_layers)| {
                if blocked {
                    return None;
                }

                // Check if camera can see this entity via RenderLayers
                let entity_layers = entity_render_layers.unwrap_or(DEFAULT_LAYERS);
                if !cam_layers.intersects(entity_layers) {
                    return None;
                }

                // Transform cursor line segment to node coordinate system
                let world_to_node = node_transform.affine().inverse();
                let cursor_start_node = world_to_node.transform_point3(cursor_ray_world.origin);
                let cursor_end_node = world_to_node.transform_point3(cursor_ray_end);

                // Find where the cursor segment intersects the plane Z=0 (which is the node's
                // plane in node-local space). It may not intersect if, for example, we're
                // viewing the node side-on
                if cursor_start_node.z == cursor_end_node.z {
                    // Cursor ray is parallel to the node and misses it
                    return None;
                }
                let lerp_factor =
                    f32::inverse_lerp(cursor_start_node.z, cursor_end_node.z, 0.0);
                if !(0.0..=1.0).contains(&lerp_factor) {
                    // Lerp factor is out of range, meaning that while an infinite line cast by
                    // the cursor would intersect the node, the node is not between the
                    // camera's near and far planes
                    return None;
                }
                // Otherwise we can interpolate the xy of the start and end positions by the
                // lerp factor to get the cursor position in node space!
                let cursor_pos_sprite = cursor_start_node
                    .lerp(cursor_end_node, lerp_factor)
                    .xy();

                let rect = Rect::from_center_size(Vec2::ZERO, **dimension);
                let is_cursor_in_sprite = rect.contains(cursor_pos_sprite);

                blocked = is_cursor_in_sprite && pickable.map(|p| p.should_block_lower).unwrap_or(true);

                is_cursor_in_sprite.then(|| {
                    let hit_pos_world =
                        node_transform.transform_point(cursor_pos_sprite.extend(0.0));
                    // Transform point from world to camera space to get the Z distance
                    let hit_pos_cam = cam_transform
                        .affine()
                        .inverse()
                        .transform_point3(hit_pos_world);
                    // HitData requires a depth as calculated from the camera's near clipping plane
                    let depth = -cam_ortho.near - hit_pos_cam.z;
                    (
                        entity,
                        HitData::new(
                            cam_entity,
                            depth,
                            Some(hit_pos_world),
                            Some(*node_transform.back()),
                        ),
                    )
                })

            })
            .collect();

        let order = camera.order as f32;
        output.write(PointerHits::new(*pointer, picks, order));
    }
}
