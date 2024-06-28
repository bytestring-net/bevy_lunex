use bevy::prelude::*;
use bevy_mod_picking::{backend::PointerHits, prelude::*};
use lunex_engine::YInvert;
use std::cmp::Ordering;
use bevy::window::PrimaryWindow;
use bevy_mod_picking::backend::prelude::*;

use crate::{Dimension, Element};


/// Adds picking support for [`bevy_lunex`].
#[derive(Clone)]
pub struct LunexBackend;
impl Plugin for LunexBackend {
    fn build(&self, app: &mut App) {
        app.add_systems(PreUpdate, lunex_picking.in_set(PickSet::Backend));
    }
}

/// Checks if any Dimension entities are under each pointer
pub fn lunex_picking(
    pointers: Query<(&PointerId, &PointerLocation)>,
    cameras: Query<(Entity, &Camera, &GlobalTransform, &OrthographicProjection)>,
    primary_window: Query<Entity, With<PrimaryWindow>>,
    node_query: Query<
        (
            Entity,
            &Dimension,
            Option<&Element>,
            &GlobalTransform,
            Option<&Pickable>,
            &ViewVisibility,
        )
    >,
    mut output: EventWriter<PointerHits>,
) {
    let mut sorted_nodes: Vec<_> = node_query.iter().collect();
    sorted_nodes.sort_by(|a, b| { (b.3.translation().z).partial_cmp(&a.3.translation().z).unwrap_or(Ordering::Equal) });

    for (pointer, location) in pointers.iter().filter_map(|(pointer, pointer_location)| { pointer_location.location().map(|loc| (pointer, loc)) }) {
        let mut blocked = false;
        let Some((cam_entity, camera, cam_transform, cam_ortho)) = cameras.iter().filter(|(_, camera, _, _)| camera.is_active)
            .find(|(_, camera, _, _)| {
                camera
                    .target
                    .normalize(Some(match primary_window.get_single() {
                        Ok(w) => w,
                        Err(_) => return false,
                    }))
                    .unwrap()
                    == location.target
            })
        else { continue; };

        let Some(cursor_pos_world) = camera.viewport_to_world_2d(cam_transform, location.position) else { continue; };

        let picks: Vec<(Entity, HitData)> = sorted_nodes
            .iter()
            .copied()
            .filter(|(.., visibility)| visibility.get())
            .filter_map(
                |(entity, dimension, element, node_transform, pickable, ..)| {
                    if blocked {
                        return None;
                    }

                    let pos = if !element.is_some() { dimension.size.invert_y() / 2.0 } else { Vec2::ZERO };

                    let rect = Rect::from_center_size(pos, dimension.size);

                    /* let s = rect.max - rect.min;
                    let p = (rect.min + s/2.0).extend(0.0) + node_transform.translation();
                    gizmos.rect(p, Quat::from_rotation_y(0.0), s, Color::linear_rgb(0.0, 0.0, 1.0)); */

                    // Transform cursor pos to sprite coordinate system
                    let cursor_pos_sprite = node_transform
                        .affine()
                        .inverse()
                        .transform_point3((cursor_pos_world, 0.0).into());

                    let is_cursor_in_sprite = rect.contains(cursor_pos_sprite.truncate());
                    blocked = is_cursor_in_sprite && pickable.map(|p| p.should_block_lower) != Some(false);

                    // HitData requires a depth as calculated from the camera's near clipping plane
                    let depth = -cam_ortho.near - node_transform.translation().z;

                    is_cursor_in_sprite.then_some((entity, HitData::new(cam_entity, depth, None, None)))
                },
            )
            .collect();

        let order = camera.order as f32;
        output.send(PointerHits::new(*pointer, picks, order));
    }
}

#[derive(Component)]
pub struct PickingPortal;

/* pub fn rendered_texture_picking(
    mut commands: Commands,
    texture_viewports: Query<(Entity, &Handle<Image>, &Dimension), With<PickingPortal>>,
    mut pointer_move: EventWriter<pointer::InputMove>,
) {

    // Draw each viewport in a window. This isn't as robust as it could be for the sake of
    // demonstration. This only works if the render target and egui texture are rendered at the same
    // resolution, and this completely ignores touch inputs and treats everything as a mouse input.
    for (viewport_entity, texture_handle, viewport_dimension) in &texture_viewports {
        if let Some(pointer_pos_window) = viewport_response.hover_pos() {
            // Compute the position of the pointer relative to the texture.
            pointer_move.send(pointer::InputMove {
                pointer_id: PointerId::Mouse,
                location: pointer::Location {
                    target: bevy::render::camera::NormalizedRenderTarget::Image(
                        texture_handle.clone_weak(),
                    ),
                    position: pointer_pos_window - viewport_dimension.size,
                },
                delta: Vec2::ZERO,
            });
        }
    }
} */