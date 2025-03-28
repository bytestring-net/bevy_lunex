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
                // spawn time animation (in this case fading out of 'collapsed' state)
                UiStateAnimation::new(vec![("collapsed", Anim::line(1., 0., 1.))]),
                UiLayout::new(vec![
                    ("base", UiLayout::window().pos(Rl(50.)).size(Rh(25.))),
                    ("hover", UiLayout::window().pos(Rl(50.)).size(Rh((80., 25.)))),
                    ("click", UiLayout::window().pos(Rl(50.)).size(Rh((80., 10.)))),
                    ("collapsed", UiLayout::window().pos(Rl(0.)).size(Rh(0.))),
                ]),
                UiColor::new(vec![
                    ("base", Color::hsla(0., 0., 0., 0.5)),
                    ("hover", Color::hsla(200., 1., 0.5, 1.0)),
                    ("click", Color::hsla(330., 1., 0.5, 1.0)),
                    ("collapsed", Color::hsla(330., 1., 2.5, 1.0)),
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
        // insert an animation for state 1 (in this case a single linear segment going from 0 to 1
        // in 0.3 secs)
        animations.insert("hover", Anim::line(0., 1., 0.3));
    }
}

fn out(
    trig: Trigger<Pointer<Out>>,
    mut query: Query<&mut UiStateAnimation>,
) {
    if let Ok(mut animations) = query.get_mut(trig.entity()) {
        // continue continues from the current weight, so if you hover out while the state isn't
        // fully active, you start fading out from the current point
        animations.insert("hover", Anim::continue_to(0., 0.8));
    }
}

fn down(
    trig: Trigger<Pointer<Down>>,
    mut query: Query<&mut UiStateAnimation>,
) {
    if let Ok(mut animations) = query.get_mut(trig.entity()) {
        // you can insert multiple stage animations
        animations.insert(
            // doing this for the "click" state
            "click",
            // (delay for 0.2 secs) then fade out (go to weight 0 in 0.5 sec)
            Anim::segs(vec![Seg::Hold(0.2), Seg::To(0., 0.5)])
                // starting with a weight of 1
                .with_init(1.)
                // make it a looping animation
                .looping(true)
        );
    }
}

fn up(
    trig: Trigger<Pointer<Up>>,
    mut query: Query<&mut UiStateAnimation>,
) {
    if let Ok(mut animations) = query.get_mut(trig.entity()) {
        animations.insert(
            "click",
            Anim::segs(vec![
                Seg::Continue(0., 1.),
                Seg::Curved(1., 0.3, downarc),
                Seg::Curved(0., 0.3, uparc),
            ]),
        );
    }
}


// test curve functions (stolen from fundsp)
fn uparc(x: f32) -> f32 {
    1. - (1. - x * x).max(0.).sqrt()
}

fn downarc(x: f32) -> f32 {
    ((2. - x) * x).max(0.).sqrt()
}
