use bevy::{prelude::*, window::SystemCursorIcon};
use bevy_lunex::prelude::*;

fn main() -> AppExit {
    App::new()
        .add_plugins((DefaultPlugins, UiLunexPlugins, UiLunexDebugPlugin::<0, 0>))
        .add_systems(Startup, setup)
        .run()
}

fn setup(
    mut commands: Commands,
) {
    commands.spawn((
        Camera2d,
        UiSourceCamera::<0>,
        Transform::from_translation(Vec3::Z * 1000.0),
    ));

    commands.spawn((
        Name::new("Root"),
        UiLayoutRoot::new_2d(),
        UiFetchFromCamera::<0>,
    )).with_children(|ui| {
        ui.spawn((
            Name::new("Boundary"),
            UiLayout::boundary()
                .pos1(Ab(20.0))
                .pos2(Rl(100.0) - Ab(20.0))
                .pack(),
        )).with_children(|ui| {
            ui.spawn((
                Name::new("Mesh"),
                // it can be used to add spawn time animations, but i'm thinking
                // about making it required by something? but not sure
                UiStateAnimation::default(),
                UiLayout::new(vec![
                    (0, UiLayout::window().pos(Rl(50.)).size(Rh(25.))),
                    (1, UiLayout::window().pos(Rl(50.)).size(Rh((80., 25.)))),
                    (2, UiLayout::window().pos(Rl(50.)).size(Rh((80., 10.)))),
                ]),
                UiColor::new(vec![
                    (0, Color::hsla(0., 0., 0., 0.5)),
                    (1, Color::hsla(200., 1., 0.5, 1.0)),
                    (2, Color::hsla(330., 1., 0.5, 1.0)),
                ]),
                UiMeshPlane2d,
                MeshMaterial2d::<ColorMaterial>::default(),
                OnHoverSetCursor::new(SystemCursorIcon::Pointer),
            )).observe(over).observe(out).observe(up).observe(down);
        });
    });
}

fn over(
    trig: Trigger<Pointer<Over>>,
    mut query: Query<&mut UiStateAnimation>,
) {
    if let Ok(mut animations) = query.get_mut(trig.entity()) {
        animations.insert(1, Anim::fade_in(0.3));
    }
}

fn out(
    trig: Trigger<Pointer<Out>>,
    mut query: Query<&mut UiStateAnimation>,
) {
    if let Ok(mut animations) = query.get_mut(trig.entity()) {
        animations.insert(1, Anim::fade_out_curved(0.8, uparc));
    }
}

fn down(
    trig: Trigger<Pointer<Down>>,
    mut query: Query<&mut UiStateAnimation>,
) {
    if let Ok(mut animations) = query.get_mut(trig.entity()) {
        animations.insert(2, Anim::fade_in_curved(0.8, downarc));
    }
}

fn up(
    trig: Trigger<Pointer<Up>>,
    mut query: Query<&mut UiStateAnimation>,
) {
    if let Ok(mut animations) = query.get_mut(trig.entity()) {
        // you can insert multiple stage animations
        animations.insert(2, Anim::segs(vec![Seg::Hold(1.), Seg::Glide(0., 1.)]));
    }
}


// test curve functions (stolen from fundsp)
fn uparc(x: f32) -> f32 {
    1. - (1. - x * x).max(0.).sqrt()
}

fn downarc(x: f32) -> f32 {
    ((2. - x) * x).max(0.).sqrt()
}
