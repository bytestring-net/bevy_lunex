use bevy::{
    input::mouse::{MouseMotion, MouseWheel},
    prelude::*,
};

#[derive(Component)]
pub struct PlayerCam {
    pub orbit: Vec3,
    pub distance: f32,
    pub sensitivity: Vec2,
}
pub fn rotate_playercam(mut mouse_motion_events: EventReader<MouseMotion>, mouse_input: Res<ButtonInput<MouseButton>>, mut query: Query<(&PlayerCam, &mut Transform)>) {
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
        rx += (-delta.x * camera.sensitivity.x).to_radians();
        ry += (-delta.y * camera.sensitivity.x).to_radians();
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
pub fn zoom_playercam(mut mouse_wheel_events: EventReader<MouseWheel>, mut query: Query<&mut PlayerCam>) {
    let delta: f32 = mouse_wheel_events.read().map(|e| e.y).sum();
    for mut camera in &mut query {
        camera.distance += -delta*25.0;
    }
}
