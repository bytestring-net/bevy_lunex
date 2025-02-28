use bevy::{input::mouse::{MouseMotion, MouseWheel}, prelude::*};


#[derive(Component, Reflect, Clone, PartialEq, Debug)]
pub struct ShowcaseCamera {
    pub orbit: Vec3,
    pub distance: f32,
    pub mouse_sensitivity: f32,
    pub zoom_scale: f32,
}
impl ShowcaseCamera {
    pub fn rotate(mut mouse_motion_events: EventReader<MouseMotion>, mouse_input: Res<ButtonInput<MouseButton>>, mut query: Query<(&ShowcaseCamera, &mut Transform)>) {
        let mut delta = Vec2::ZERO;
        if mouse_input.pressed(MouseButton::Left) {
            delta = mouse_motion_events.read().map(|e| e.delta).sum();
        }
        if mouse_input.just_pressed(MouseButton::Left) {
            delta = Vec2::ZERO;
        }
        for (camera, mut transform) in &mut query {

            // ROTATION 
            let (mut rx, mut ry, rz) = transform.rotation.to_euler(EulerRot::YXZ);
            rx += (-delta.x * camera.mouse_sensitivity).to_radians();
            ry += (-delta.y * camera.mouse_sensitivity).to_radians();
            ry = ry.clamp(-90_f32.to_radians(), 90_f32.to_radians());
            transform.rotation = Quat::from_euler(EulerRot::YXZ, rx, ry, rz);


            // ORBIT TRANSFORM
            let tx = camera.distance * rx.sin();
            let ty = camera.distance * rx.cos();
            let tz = camera.distance * ry.sin();

            let diff = camera.distance * ry.cos();
            let plane_ratio_decrease = (camera.distance - diff)/camera.distance;

            transform.translation = camera.orbit;
            transform.translation.x += tx * (1.0 - plane_ratio_decrease);
            transform.translation.z += ty * (1.0 - plane_ratio_decrease);
            transform.translation.y += -tz;
        }
    }
    pub fn zoom(mut mouse_wheel_events: EventReader<MouseWheel>, mut query: Query<&mut ShowcaseCamera>) {
        let delta: f32 = mouse_wheel_events.read().map(|e| e.y).sum();
        for mut camera in &mut query {
            if camera.zoom_scale != 0.0 {
                camera.distance -= delta * camera.distance/camera.zoom_scale;
            }
        }
    }
}
impl Default for ShowcaseCamera {
    fn default() -> Self {
        Self {
            orbit: Vec3::ZERO,
            distance: 2.0,
            mouse_sensitivity: 0.1,
            zoom_scale: 25.0,
        }
    }
}
